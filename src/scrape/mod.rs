use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::{alpha1, char, multispace0, multispace1},
    combinator::{fail, map, map_res, opt},
    sequence::{delimited, pair},
    IResult, Parser,
};
use std::{collections::HashMap, str::FromStr};
use tracing::instrument;

// pub struct EnvMap(HashMap<String, Expr>);

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
}

impl Typed for Value {
    type TypeRepr = TypeRepr;
    type E = ();

    fn get_type(s: Self) -> Result<TypeRepr, Self::E> {
        match s {
            Self::Number(_) => Ok(TypeRepr::Number),
            Self::Bool(_) => Ok(TypeRepr::Bool),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeRepr {
    Number,
    Bool,
}

trait Typed {
    type TypeRepr;
    type E;

    fn get_type(s: Self) -> Result<Self::TypeRepr, Self::E>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    Binop(Oper, Box<Expr>, Box<Expr>),
    //  let name = e1 in e2
    Let(String, Box<Expr>, Box<Expr>),

    // Will be substituded. Only usable in let expr
    Ident(String),
}

impl Typed for Expr {
    type TypeRepr = TypeRepr;
    type E = ();

    fn get_type(s: Self) -> Result<Self::TypeRepr, Self::E> {
        match s {
            Self::Value(v) => Ok(Value::get_type(v)?),
            Self::Binop(o, _, _) => Ok(Oper::get_type(o)?), // TODO typecheck args
            Self::Let(_, _, e2) => Ok(Expr::get_type(*e2)?),
            Self::Ident(_) => unreachable!(),
        }
    }
}

/// The order of AST Expr Nodes represents the structure of parentheses and therefore Expr has no
/// Paren variant.
///
/// # Example
///
/// ```rust
/// use seekr_rs::scrape::{Expr, parse_parens, Value, Oper};
/// use std::collections::HashMap;
/// let result = parse_parens(HashMap::new(), "(4+20)");
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

pub fn parse_parens(envo: HashMap<String, Expr>, i: &str) -> IResult<&str, Expr> {
    delimited(char('('), |i| parse_expr(envo.clone(), i), char(')')).parse(i)
}

/// Expr : 4 * 20
///        ^
///        Factor
/// e.g. 4 * 20 + 6 --> (4 * 20) + 6
///      4 * 20 * 6 --> (4 * 20) * 6
///      4 * (20 + 6) --> 4  * (20 + 6)
///  therefore parse_parens parses an expression inside parens.

#[instrument]
pub fn parse_factor(envo: HashMap<String, Expr>, i: &str) -> IResult<&str, Expr> {
    alt((
        map(parse_value, Expr::Value),
        |i| parse_parens(envo.clone(), i),
        |i| {
            // NOTE really weird type stuff
            let (i, _) = multispace0(i)?;
            let (i, name): (&str, &str) = alpha1(i)?;
            if let Some(e) = envo.get(name) {
                Ok((i, e.to_owned()))
            } else {
                fail(i)
            }
        },
    ))
    .parse(i)
}

pub fn parse_term_aux(envo: HashMap<String, Expr>, i: &str, factor: Expr) -> IResult<&str, Expr> {
    if let Ok((i, (op, remainder))) =
        pair(map_res(alt((tag("*"), tag("/"))), FromStr::from_str), |i| {
            parse_factor(envo.clone(), i)
        })(i)
    {
        let res = Expr::Binop(op, Box::new(factor), Box::new(remainder));
        Ok(parse_term_aux(envo.clone(), i, res.clone()).unwrap_or((i, res)))
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
pub fn parse_term(envo: HashMap<String, Expr>, i: &str) -> IResult<&str, Expr> {
    let (i, factor) = parse_factor(envo.clone(), i)?;
    parse_term_aux(envo.clone(), i, factor)
}

#[instrument]
pub fn parse_expr_aux(envo: HashMap<String, Expr>, i: &str, initial: Expr) -> IResult<&str, Expr> {
    if let Ok((i, (op, remainder))) =
        pair(map_res(alt((tag("+"), tag("-"))), FromStr::from_str), |i| {
            parse_term(envo.clone(), i)
        })(i)
    {
        let res = Expr::Binop(op, Box::new(initial), Box::new(remainder));
        Ok(parse_expr_aux(envo.clone(), i, res.clone()).unwrap_or((i, res)))
    } else {
        Ok((i, initial))
    }
}
// /// ```rust
// /// use seekr_rs::scrape::{Expr, parse_parens, Value, Oper};
// /// ```
// fn parse_tags(tags: Vec<String>, i: &str) -> IResult<&str, &str> {
//     for p in tags {
//         if tag::<&str, &str, dyn nom::error::ParseError<&str>>(p.as_str())(i).is_ok() {
//             break;
//         }
//     }

//     // let parser = map(alt(parsers), |matched_tag| matched_tag.trim());

//     // parser(input)
//     nom::combinator::success(i)(i)
// }

#[instrument]
pub fn parse_expr(envo: HashMap<String, Expr>, i: &str) -> IResult<&str, Expr> {
    alt((
        |i| {
            let (i, initial) = parse_term(envo.clone(), i)?;
            parse_expr_aux(envo.clone(), i, initial)
        },
        |i| {
            let (i, _) = delimited(multispace0, tag("let"), multispace1)(i)?;
            let (i, name) = alpha1(i)?;
            let (i, _) = delimited(multispace0, tag("="), multispace0)(i)?;
            let (i, e1) = parse_expr(envo.clone(), i)?;
            let (i, _) = delimited(multispace1, tag("in"), multispace1)(i)?;
            let mut envomut = envo.clone();
            envomut.insert(name.to_string(), e1.clone());
            // let envo = envomut.clone();
            let (i, e2) = parse_expr(envomut, i)?;
            // TODO subst

            Ok((i, Expr::Let(name.to_string(), Box::new(e1), Box::new(e2))))
        },
    ))
    .parse(i)
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
    /// 4 == 4
    Eq, // TODO
    /// 4 != 4
    Neq, // TODO
}

impl Typed for Oper {
    type TypeRepr = TypeRepr;
    type E = ();

    fn get_type(s: Self) -> Result<Self::TypeRepr, Self::E> {
        match s {
            Self::Add | Self::Sub | Self::Mul | Self::Div => Ok(TypeRepr::Number),
            Self::Eq | Self::Neq => Ok(TypeRepr::Bool),
        }
    }
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
pub fn parse_value(i: &str) -> IResult<&str, Value> {
    // No need for env
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
    use std::collections::hash_map::HashMap;
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
    use seekr_macro::value;
    use tracing_test::traced_test;
    #[traced_test]
    #[test]
    fn test_parse_expr() {
        // expr!(Add,
        //     value!(1)
        //     value!(2)
        // )

        // value!(int_one!());
        assert_eq!(
            parse_expr(HashMap::new(), "1*3+-2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(Oper::Mul, value!(1), value!(3))),
                    value!(-2)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "1*3+2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(Oper::Mul, value!(1), value!(3))),
                    value!(2)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "1*3+(2)"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(Oper::Mul, value!(1), value!(3))),
                    value!(2)
                )
            ))
        );
        assert_eq!(
            parse_expr(HashMap::new(), "true"),
            Ok(("", Expr::Value(Value::Bool(true))))
        );
        assert_eq!(
            parse_expr(HashMap::new(), "false"),
            Ok(("", Expr::Value(Value::Bool(false))))
        );
        for i in -1000..1000 {
            assert_eq!(
                parse_expr(HashMap::new(), format!("{}", i).as_str()),
                Ok(("", Expr::Value(Value::Number(i))))
            );
        }

        assert_eq!(
            parse_expr(HashMap::new(), "1+2+5"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(Oper::Add, value!(1), value!(2))),
                    value!(5)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "1*2*5"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(Oper::Mul, value!(1), value!(2))),
                    value!(5)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "1*2*5-2"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Sub,
                    Box::new(Expr::Binop(
                        Oper::Mul,
                        Box::new(Expr::Binop(Oper::Mul, value!(1), value!(2))),
                        value!(5)
                    )),
                    value!(2)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "1*2*5-2+1"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    Box::new(Expr::Binop(
                        Oper::Sub,
                        Box::new(Expr::Binop(
                            Oper::Mul,
                            Box::new(Expr::Binop(Oper::Mul, value!(1), value!(2))),
                            value!(5)
                        )),
                        value!(2)
                    )),
                    value!(1)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "(1+2)*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(Oper::Add, value!(1), value!(2))),
                    value!(3)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "(1+2+3)*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Mul,
                    Box::new(Expr::Binop(
                        Oper::Add,
                        Box::new(Expr::Binop(Oper::Add, value!(1), value!(2))),
                        value!(3),
                    )),
                    value!(3)
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "let foo=2 in 3"),
            Ok(("", Expr::Let("foo".to_string(), value!(2), value!(3))))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "let foo=2 in foo"),
            Ok(("", Expr::Let("foo".to_string(), value!(2), value!(2))))
        );
        assert_eq!(
            parse_expr(HashMap::new(), "let foo=2 in foo+3"),
            Ok((
                "",
                Expr::Let(
                    "foo".to_string(),
                    value!(2),
                    Box::new(Expr::Binop(Oper::Add, value!(2), value!(3)))
                )
            ))
        );

        assert_eq!(
            parse_expr(HashMap::new(), "let foo=2 in let foo = foo+3 in foo-1"),
            Ok((
                "",
                Expr::Let(
                    "foo".to_string(),
                    value!(2),
                    Box::new(Expr::Let(
                        "foo".to_string(),
                        Box::new(Expr::Binop(Oper::Add, value!(2), value!(3))),
                        Box::new(Expr::Binop(
                            Oper::Sub,
                            Box::new(Expr::Binop(Oper::Add, value!(2), value!(3))),
                            value!(1)
                        ))
                    ))
                )
            ))
        );
        assert_eq!(
            parse_expr(HashMap::new(), "1+2*3"),
            Ok((
                "",
                Expr::Binop(
                    Oper::Add,
                    value!(1),
                    Box::new(Expr::Binop(Oper::Mul, value!(2), value!(3)))
                )
            ))
        );
    }
}
