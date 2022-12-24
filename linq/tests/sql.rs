use linq_proc_macro::*;
use linq_rs::{
    dml::*,
    orm::{Select, Table},
    QueryIterator, SelectSupport, Variant,
};

#[async_std::test]
async fn test_select() {
    let id_col_name = "id";
    let no_col_name = "user_social_no";

    let (qir, _) = rqls! {
        SELECT name,#id_col_name, no AS #no_col_name FROM table;
        SELECT name,#id_col_name, no AS #no_col_name FROM table;
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
        SELECT #(cols)* FROM table;
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec!["name".into(), "id".into()])
    );

    let cols = vec![("name", "user_name"), ("id", "user_id")];

    let qir = rql! {
        SELECT #(cols)* FROM table;
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec![("name", "user_name").into(), ("id", "user_id").into()])
    );
}

#[async_std::test]
async fn test_cond() {
    let qir = rql! {
         SELECT * FROM table WHERE id != 100 AND (name = "hello" OR name = "world");
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
        SELECT * FROM table WHERE id >= 100;
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
        SELECT * FROM table WHERE id <= 100;
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
        SELECT * FROM table WHERE id > 100;
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
        SELECT * FROM table WHERE id < 100;
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
       SELECT * FROM table WHERE id in (100,200,300);
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
        SELECT * FROM table WHERE name LIKE "%hello%";
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
        SELECT * FROM table LIMIT #limit OFFSET #offset;
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
        SELECT * FROM table ORDER BY #col_name;
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        SELECT * FROM table ORDER BY #col_name ASC;
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        SELECT * FROM table ORDER BY #col_name DESC;
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
        SELECT * FROM table ORDER BY #col_name #desc;
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
        SELECT #(cols)* FROM table WHERE id = #id ORDER BY name DESC LIMIT 10 OFFSET 2;
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

#[test]
fn test_insert() {
    let qir = rql! {
        INSERT INTO table(name,content)
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());

    let col_name = "name_2".to_owned();

    let qir = rql! {
        INSERT INTO table(#col_name.as_str(),content)
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name_2", "content"].into());

    let cols = &["name", "content"];

    let qir = rql! {
        INSERT INTO table #(cols)*
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());
}

#[test]
fn test_update() {
    let qir = rql! {
        UPDATE table(name,content) WHERE id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());

    let col_name = "name_2".to_owned();

    let qir = rql! {
        UPDATE table(#col_name.as_str(),content) WHERE id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name_2", "content"].into());

    let cols = &["name", "content"];

    let qir = rql! {
        UPDATE table #(cols)* WHERE id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());
}

#[test]
fn test_delete() {
    let table_name = "hello";
    let qir = rql! {
        DELETE FROM #table_name  WHERE id = 1
    };

    assert_eq!(qir.table_name, "hello");

    assert_eq!(
        qir.cond,
        CondExpr {
            op: CondOp::Eq,
            lhs: CondParam::Variant("id".into()),
            rhs: CondParam::Variant(1.into()),
        }
    );
}
