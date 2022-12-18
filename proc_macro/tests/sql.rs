use linq_proc_macro::*;
use linq_rs_ir::Variant;

#[async_std::test]
async fn test_select() {
    let order_by = vec!["name".to_owned()];

    rql! {
        select *
        where id = 100 and (name = "hello" or name = "test")
        order by name
        limit 10 offset 3
    };

    let selecter = rql! {
        select name as #[ async { format!("col_{}",order_by[0]) } ]
        where id = 100 and (name = "hello" or name = "test")
        order by name
        limit 10
    };

    assert_eq!(selecter.cols.len(), 1);

    assert_eq!(selecter.cols[0].col_name, Variant::Constant("name"));

    assert_eq!(
        selecter.cols[0].aliase,
        Some(Variant::Eval("col_name".to_owned()))
    );

    rql! {
        select *
        where id = 100 and (name = "hello" or name = "test")
        order by name
    };

    let _ql = rql! {
        select name as #[ order_by[0].as_str() ] , id
        where id = 100 and (name = "hello" or name = "test")
    };

    let _ql = rql! {
        select *
    };
}
