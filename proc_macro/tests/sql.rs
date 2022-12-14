use linq_proc_macro::*;

#[test]
fn test_select() {
    rql! {
        select * from user
        where
        id = 100 and
        (name = "hello" or name = "test")
    };
}
