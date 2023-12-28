use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag},
    character::complete::{alpha1, char, multispace0, multispace1},
    character::complete::{digit1, one_of},
    combinator::{cut, fail, map, map_res, opt, value},
    error::{context, ContextError, ParseError},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use std::{collections::HashMap, str::FromStr};
use tracing::instrument;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
    Str(String),
}

impl Typed for Value {
    type TypeRepr = TypeRepr;
    type E = ();

    fn get_type(s: Self) -> Result<TypeRepr, Self::E> {
        match s {
            Self::Number(_) => Ok(TypeRepr::Number),
            Self::Bool(_) => Ok(TypeRepr::Bool),
            Self::Str(_) => Ok(TypeRepr::Str),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TypeRepr {
    Number,
    Bool,
    Str,
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
    If(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Typed for Expr {
    type TypeRepr = TypeRepr;
    type E = ();

    fn get_type(s: Self) -> Result<Self::TypeRepr, Self::E> {
        match s {
            Self::Value(v) => Ok(Value::get_type(v)?),
            Self::Binop(o, _, _) => Ok(Oper::get_type(o)?), // TODO typecheck args
            Self::Let(_, _, e2) => Ok(Expr::get_type(*e2)?),
            Self::If(_, _, e2) => Ok(Expr::get_type(*e2)?),
        }
    }
}
// pub enum TypeCheckResult {
//     Failed(String),
//     Succeeded
// }
impl Oper {
    pub fn get_types_to_check(&self) -> (TypeRepr, TypeRepr) {
        match self {
            Oper::Add | Oper::Sub | Oper::Mul | Oper::Div => (TypeRepr::Number, TypeRepr::Number),
            Oper::Eq | Oper::Neq => (TypeRepr::Bool, TypeRepr::Bool),
        }
    }
}
pub trait TypeCheck {
    type R;

    fn typecheck(&self) -> Self::R;
}

impl TypeCheck for Expr {
    type R = Result<(), ()>;

    fn typecheck(&self) -> Self::R {
        match self {
            Self::Value(_) => Ok(()),
            Self::If(condition, e1, e2) => {
                if Expr::get_type(*condition.clone())? == TypeRepr::Bool
                    && Expr::get_type(*e1.clone())? == Expr::get_type(*e2.clone())?
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
            Self::Binop(op, e1, e2) => {
                if op.get_types_to_check()
                    == (Expr::get_type(*e1.clone())?, Expr::get_type(*e2.clone())?)
                {
                    Ok(())
                } else {
                    Err(())
                }
            }
            Self::Let(_, _, _) => Ok(()),
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
        |i| {
            let (i, _) = delimited(multispace0, tag("if"), multispace1)(i)?;
            let (i, cond) = parse_expr(envo.clone(), i)?;
            let (i, _) = delimited(multispace1, tag("then"), multispace1)(i)?;
            let (i, e1) = parse_expr(envo.clone(), i)?;
            let (i, _) = delimited(multispace1, tag("else"), multispace1)(i)?;
            let (i, e2) = parse_expr(envo.clone(), i)?;

            Ok((i, Expr::If(Box::new(cond), Box::new(e1), Box::new(e2))))
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

// original: !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~
// modified: ! #$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[ ]^_`abcdefghijklmnopqrstuvwxyz{|}~
fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, String, E> {
    Ok(escaped_transform(
        one_of(
            r#" ! #$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[ ]^_`abcdefghijklmnopqrstuvwxyz{|}~"#,
        ),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
        )),
    )(i)?)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, String, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

pub fn parse_value(i: &str) -> IResult<&str, Value> {
    // No need for env since values are always literals
    alt((
        map(
            pair(opt(char('-')), map_res(digit1, |s: &str| s.parse::<i64>())),
            |(sign, number)| Value::Number(if sign.is_some() { -number } else { number }),
        ),
        map(
            map_res(alt((tag("true"), tag("false"))), FromStr::from_str),
            Value::Bool,
        ),
        map(string, Value::Str),
    ))
    .parse(i)
}

mod test;
