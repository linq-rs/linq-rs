use linq_proc_macro::*;
use linq_rs_ir::dml::{CondExpr, CondOp, CondParam, Limit, OrderBy, SelectColumns};

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
        where id != 100 and (name = "hello" or name = "world") select *
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
         where id >= 100 select *
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
        where id <= 100 select *
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
         where id > 100 select *
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
        where id < 100 select *
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
       where id in (100,200,300) select *
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
         where name like "%hello%" select *
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

#[test]
fn test_limit() {
    let limit = 10;
    let offset = 20;

    let qir = rql! {
        limit #limit offset #offset select *
    };

    assert_eq!(
        qir.limit,
        Some(Limit {
            count: 10,
            offset: Some(20)
        })
    );
}

#[test]
fn test_order() {
    let col_name = "hello";

    let qir = rql! {
        order by #col_name select *
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        order by #col_name asc select *
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        order by #col_name desc select *
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: true
        })
    );

    let desc = true;

    let qir = rql! {
        order by #col_name #desc select *
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: true
        })
    );
}

#[test]
fn test_select_where_order_limits() {
    let cols = vec!["name", "id"];

    let id = 100;

    let qir = rql! {
        where id = #id order by name desc limit 10 offset 2 select #(cols)*
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec!["name".into(), "id".into()])
    );

    assert_eq!(
        qir.limit,
        Some(Limit {
            count: 10,
            offset: Some(2)
        })
    );

    assert_eq!(
        qir.cond,
        Some(CondExpr {
            op: CondOp::Eq,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(100.into()),
        })
    );

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "name",
            desc: true
        })
    );
}
