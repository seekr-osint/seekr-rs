use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::digit1,
    combinator::{map, map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};
use std::str::FromStr;
use tracing::instrument;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    Binop(Oper, Box<Expr>, Box<Expr>),
}

/// The order of AST Expr Nodes represents the structure of parentheses and therefore Expr has no
/// Paren variant.
///
/// # Example
///
/// ```rust
/// use seekr_rs::scrape::{Expr, parse_parens, Value, Oper};
/// let result = parse_parens("(4+20)");
/// assert_eq!(
///     result,
///     Ok((
///         "",
///         Expr::Binop(
///             Oper::Add,
///             Box::new(Expr::Value(Value::Number(4))),
///             Box::new(Expr::Value(Value::Number(20)))
///         )
///     ))
/// );
/// ```
pub fn parse_parens(i: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr, char(')')).parse(i)
}

/// Expr : 4 * 20
///        ^
///        Factor
/// e.g. 4 * 20 + 6 --> (4 * 20) + 6
///      4 * 20 * 6 --> (4 * 20) * 6
///      4 * (20 + 6) --> 4  * (20 + 6)
///  therefore parse_parens parses an expression inside parens.

#[instrument]
pub fn parse_factor(i: &str) -> IResult<&str, Expr> {
    alt((map(parse_value, Expr::Value), parse_parens)).parse(i)
}

pub fn parse_term_aux(i: &str, factor: Expr) -> IResult<&str, Expr> {
    if let Ok((i, (op, remainder))) = pair(
        map_res(alt((tag("*"), tag("/"))), FromStr::from_str),
        parse_factor,
    )(i)
    {
        let res = Expr::Binop(op, Box::new(factor), Box::new(remainder));
        Ok(parse_term_aux(i, res.clone()).unwrap_or((i, res)))
    } else {
        Ok((i, factor))
    }
}

/// Expr : 4 * 20
///        ^^^^^^
///        Term
/// Expr: 4 * 20
///           ^ (remainder : factor)
/// Expr: 4 / 20
///           ^ (remainder : factor)
/// A term is multiplication and division of to factors.
/// In `[parse_factor]` is explained why the remainder is a factor and not an expression.
#[instrument]
pub fn parse_term(i: &str) -> IResult<&str, Expr> {
    let (i, factor) = parse_factor(i)?;
    parse_term_aux(i, factor)
}

#[instrument]
pub fn parse_expr_aux(i: &str, initial: Expr) -> IResult<&str, Expr> {
    if let Ok((i, (op, remainder))) = pair(
        map_res(alt((tag("+"), tag("-"))), FromStr::from_str),
        parse_term,
    )(i)
    {
        let res = Expr::Binop(op, Box::new(initial), Box::new(remainder));
        Ok(parse_expr_aux(i, res.clone()).unwrap_or((i, res)))
    } else {
        Ok((i, initial))
    }
}

#[instrument]
pub fn parse_expr(i: &str) -> IResult<&str, Expr> {
    let (i, initial) = parse_term(i)?;
    parse_expr_aux(i, initial)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Oper {
    /// 4 + 20
    Add,
    /// 4 - 20
    Sub,
    /// 4 * 20
    Mul,
    /// 4 / 20
    Div,
}
impl FromStr for Oper {
    type Err = &'static str;

    #[instrument]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err("Invalid Operator"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
}
pub fn parse_value(i: &str) -> IResult<&str, Value> {
    alt((
        map(
            pair(opt(char('-')), map_res(digit1, |s: &str| s.parse::<i64>())),
            |(sign, number)| Value::Number(if sign.is_some() { -number } else { number }),
        ),
        map(
            map_res(alt((tag("true"), tag("false"))), FromStr::from_str),
            Value::Bool,
        ),
    ))
    .parse(i)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("true"), Ok(("", Value::Bool(true))));
        assert_eq!(parse_value("false"), Ok(("", Value::Bool(false))));
        for i in -1000..1000 {
            assert_eq!(
                parse_value(format!("{}", i).as_str()),
                Ok(("", Value::Number(i)))
            );
        }
    }
    use tracing_test::traced_test;
    #[traced_test]
    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_expr("1*3+-2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(3)))
                    )),
                    Box::new(Expr::Value(Value::Number(-2)))
                )
            ))
        );

        assert_eq!(
            parse_expr("1*3+2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(3)))
                    )),
                    Box::new(Expr::Value(Value::Number(2)))
                )
            ))
        );

        assert_eq!(
            parse_expr("1*3+(2)"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(3)))
                    )),
                    Box::new(Expr::Value(Value::Number(2)))
                )
            ))
        );
        assert_eq!(parse_expr("true"), Ok(("", Expr::Value(Value::Bool(true)))));
        assert_eq!(
            parse_expr("false"),
            Ok(("", Expr::Value(Value::Bool(false))))
        );
        for i in -1000..1000 {
            assert_eq!(
                parse_expr(format!("{}", i).as_str()),
                Ok(("", Expr::Value(Value::Number(i))))
            );
        }

        assert_eq!(
            parse_expr("1+2+5"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Add,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(2)))
                    )),
                    Box::new(Expr::Value(Value::Number(5)))
                )
            ))
        );

        assert_eq!(
            parse_expr("1*2*5"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(2)))
                    )),
                    Box::new(Expr::Value(Value::Number(5)))
                )
            ))
        );

        assert_eq!(
            parse_expr("1*2*5-2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Sub,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Binop(
                            Oper::Mul,
                            Box::new(Expr::Value(Value::Number(1))),
                            Box::new(Expr::Value(Value::Number(2)))
                        )),
                        Box::new(Expr::Value(Value::Number(5)))
                    )),
                    Box::new(Expr::Value(Value::Number(2)))
                )
            ))
        );

        assert_eq!(
            parse_expr("1*2*5-2+1"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Sub,
                        Box::new(Expr::Binop(
                            Oper::Mul,
                            Box::new(Expr::Binop(
                                Oper::Mul,
                                Box::new(Expr::Value(Value::Number(1))),
                                Box::new(Expr::Value(Value::Number(2)))
                            )),
                            Box::new(Expr::Value(Value::Number(5)))
                        )),
                        Box::new(Expr::Value(Value::Number(2)))
                    )),
                    Box::new(Expr::Value(Value::Number(1)))
                )
            ))
        );

        assert_eq!(
            parse_expr("(1+2)*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(
                        Oper::Add,
                        Box::new(Expr::Value(Value::Number(1))),
                        Box::new(Expr::Value(Value::Number(2)))
                    )),
                    Box::new(Expr::Value(Value::Number(3)))
                )
            ))
        );

        assert_eq!(
            parse_expr("(1+2+3)*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(
                        Oper::Add,
                        Box::new(Expr::Binop(
                            Oper::Add,
                            Box::new(Expr::Value(Value::Number(1))),
                            Box::new(Expr::Value(Value::Number(2)))
                        )),
                        Box::new(Expr::Value(Value::Number(3))),
                    )),
                    Box::new(Expr::Value(Value::Number(3)))
                )
            ))
        );
        assert_eq!(
            parse_expr("1+2*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Value(Value::Number(1))),
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Value(Value::Number(2))),
                        Box::new(Expr::Value(Value::Number(3)))
                    ))
                )
            ))
        );
    }
}
