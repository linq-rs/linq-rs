use linq_proc_macro::*;
use linq_rs_ir::dml::{CondExpr, CondOp, CondParam, SelectColumns};

#[async_std::test]
async fn test_select() {
    let id_col_name = "id";
    let no_col_name = "user_social_no";

    let qir = rql! {
        select name,#id_col_name, no as #no_col_name
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec![
            "name".into(),
            "id".into(),
            ("no", "user_social_no").into()
        ])
    );

    let cols = vec!["name", "id"];

    let qir = rql! {
        select #(cols)*
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec!["name".into(), "id".into()])
    );

    let cols = vec![("name", "user_name"), ("id", "user_id")];

    let qir = rql! {
        select #(cols)*
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec![("name", "user_name").into(), ("id", "user_id").into()])
    );
}

#[async_std::test]
async fn test_cond() {
    let qir = rql! {
        select * where id != 100 and (name = "hello" or name = "world")
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::And,
            lhs: CondParam::CondExpr(Box::new(CondExpr {
                op: CondOp::NotEq,
                lhs: CondParam::Variant("id".into()),
                rhs: CondParam::Variant(100.into()),
            })),
            rhs: CondParam::CondExpr(Box::new(CondExpr {
                op: CondOp::Or,
                lhs: CondParam::CondExpr(Box::new(CondExpr {
                    op: CondOp::Eq,
                    lhs: CondParam::Variant("name".into()),
                    rhs: CondParam::Variant("hello".into()),
                })),
                rhs: CondParam::CondExpr(Box::new(CondExpr {
                    op: CondOp::Eq,
                    lhs: CondParam::Variant("name".into()),
                    rhs: CondParam::Variant("world".into()),
                })),
            }))
        })
    );

    let qir = rql! {
        select * where id >= 100
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Gte,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(100.into()),
        })
    );

    let qir = rql! {
        select * where id <= 100
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Lte,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(100.into()),
        })
    );

    let qir = rql! {
        select * where id > 100
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Gt,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(100.into()),
        })
    );

    let qir = rql! {
        select * where id < 100
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Lt,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(100.into()),
        })
    );

    let qir = rql! {
        select * where id in (100,200,300)
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::In,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::VariantList(vec![100.into(), 200.into(), 300.into()]),
        })
    );

    let qir = rql! {
        select * where name like "%hello%"
    };

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Like,
            lhs: CondParam::Variant("name".into()),
            rhs: CondParam::Variant("%hello%".into()),
        })
    );
}
