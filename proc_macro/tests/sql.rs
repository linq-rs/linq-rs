use linq_proc_macro::*;

#[test]
fn test_select() {
    let order_by = "name";

    rql! {
        select *
        where id = 100 and (name = "hello" or name = "test")
        order by name
        limit 10 offset 3
    };

    rql! {
        select *
        where id = 100 and (name = "hello" or name = "test")
        order by || format!("col_{}",order_by)
        limit 10
    };

    rql! {
        select *
        where id = 100 and (name = "hello" or name = "test")
        order by name
    };

    rql! {
        select name as || format!("{}",order_by) , id
        where id = 100 and (name = "hello" or name = "test")
    };

    rql! {
        select *
    };
}
