use linq_proc_macro::*;
use linq_rs::dml::*;

#[async_std::test]
async fn test_select() {
    let id_col_name = "id";
    let no_col_name = "user_social_no";

    let (qir, _) = rqls! {
        select name,#id_col_name, no as #no_col_name from table;
        select name,#id_col_name, no as #no_col_name from table;
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
        select #(cols)* from table;
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec!["name".into(), "id".into()])
    );

    let cols = vec![("name", "user_name"), ("id", "user_id")];

    let qir = rql! {
        select #(cols)* from table;
    };

    assert_eq!(
        qir.cols,
        SelectColumns::NamedColumns(vec![("name", "user_name").into(), ("id", "user_id").into()])
    );
}

#[async_std::test]
async fn test_cond() {
    let qir = rql! {
         select * from table where id != 100 and (name = "hello" or name = "world");
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
        select * from table where id >= 100;
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
        select * from table where id <= 100;
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
        select * from table where id > 100;
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
        select * from table where id < 100;
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
       select * from table where id in (100,200,300);
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
        select * from table where name like "%hello%";
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
        select * from table limit #limit offset #offset;
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
        select * from table order by #col_name;
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        select * from table order by #col_name asc;
    };

    assert_eq!(
        qir.order_by,
        Some(OrderBy {
            col_name: "hello",
            desc: false
        })
    );

    let qir = rql! {
        select * from table order by #col_name desc;
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
        select * from table order by #col_name #desc;
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
        select #(cols)* from table where id = #id order by name desc limit 10 offset 2;
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
        insert into table(name,content)
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());

    let col_name = "name_2".to_owned();

    let qir = rql! {
        insert into table(#col_name.as_str(),content)
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name_2", "content"].into());

    let cols = &["name", "content"];

    let qir = rql! {
        insert into table #(cols)*
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());
}

#[test]
fn test_update() {
    let qir = rql! {
        update table(name,content) where id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());

    let col_name = "name_2".to_owned();

    let qir = rql! {
        update table(#col_name.as_str(),content) where id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name_2", "content"].into());

    let cols = &["name", "content"];

    let qir = rql! {
        update table #(cols)* where id = 1
    };

    assert_eq!(qir.table_name, "table");

    assert_eq!(qir.cols, vec!["name", "content"].into());
}

#[test]
fn test_delete() {
    let table_name = "hello";
    let qir = rql! {
        delete from #table_name  where id = 1
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
