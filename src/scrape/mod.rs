use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::{char, one_of},
    combinator::{map, map_res, opt},
    sequence::{delimited, pair, separated_pair},
    IResult, Parser,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    Binop(Oper, Box<Expr>, Box<Expr>),
    Paren(Box<Expr>),
}

pub fn parse_expr(i: &str) -> IResult<&str, Expr> {
    alt((
        map(delimited(char('('), parse_expr, char(')')), |e| {
            Expr::Paren(Box::new(e))
        }), // TODO
        map(
            separated_pair(parse_expr, char('+'), parse_expr),
            |(e1, e2)| Expr::Binop(Oper::Add, Box::new(e1), Box::new(e2)),
        ), // TODO
        map(parse_value, Expr::Value),
    ))
    .parse(i)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Oper {
    Add,
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
    #[test]
    fn test_parse_expr() {
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

        // let one = Expr::Value(Value::Number(1));
        // assert_eq!(parse_expr("(1)+(1)"), Ok(("", Expr::Binop(Oper::Add,Box::new(one.clone()),Box::new(one.clone())))));
    }
}
