#![cfg(test)]
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
        parse_expr(HashMap::new(), "if true then 1 else 2"),
        Ok(("", Expr::If(value!(true), value!(1), value!(2),)))
    );

    assert_eq!(
        parse_expr(HashMap::new(), "let foo = if true then 1 else 2 in foo"),
        Ok((
            "",
            Expr::Let(
                "foo".to_string(),
                Box::new(Expr::If(value!(true), value!(1), value!(2))),
                Box::new(Expr::If(value!(true), value!(1), value!(2))),
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

    let t = vec![
        "hello",
        "world",
        "        let generated_string: String = string_strategy.generate(&mut rng);",
        r#"! #$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[ ]^_`abcdefghijklmnopqrstuvwxyz{|}~"#,
    ];

    for t1 in t {
        assert_eq!(
            parse_expr(HashMap::new(), &format!(r#""{}""#, t1)),
            Ok(("", Expr::Value(Value::Str(t1.to_string()))))
        );
    }

    assert_eq!(
        parse_expr(HashMap::new(), r#""Hello World""#),
        Ok(("", Expr::Value(Value::Str("Hello World".to_string()))))
    );
    assert_eq!(
        parse_expr(HashMap::new(), r#""+2*3""#),
        Ok(("", Expr::Value(Value::Str("+2*3".to_string()))))
    );

    assert_eq!(
        parse_expr(HashMap::new(), r#""\"""#),
        Ok(("", Expr::Value(Value::Str(r#"""#.to_string()))))
    );

    assert_eq!(
        parse_expr(HashMap::new(), r#""+2*3""#),
        Ok(("", Expr::Value(Value::Str("+2*3".to_string()))))
    );

    assert_eq!(
        parse_expr(
            HashMap::new(),
            r#""! #$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~""#
        ),
        Ok(("", 

                Expr::Value(Value::Str( r#"! #$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~"# .to_string()))))
    );
}

#[traced_test]
#[test]
fn test_typecheck() {
    let (_, res) = parse_expr(HashMap::new(), "1+2+5").unwrap();
    assert_eq!(res.typecheck(), Ok(()));

    let (_, res) = parse_expr(HashMap::new(), r#""hello world""#).unwrap();
    assert_eq!(res.typecheck(), Ok(()));

    let (_, res) = parse_expr(HashMap::new(), r#""hello world"+1"#).unwrap();
    assert_eq!(res.typecheck(), Err(()));

    let (_, res) = parse_expr(HashMap::new(), r#"if "hello world" then 1 else 2"#).unwrap();
    assert_eq!(res.typecheck(), Err(()));

    let (_, res) = parse_expr(HashMap::new(), r#"if false then 1 else 2"#).unwrap();
    assert_eq!(res.typecheck(), Ok(()));

    let (_, res) = parse_expr(HashMap::new(), r#"if false then false else 2"#).unwrap();
    assert_eq!(res.typecheck(), Err(()));
}
