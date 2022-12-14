use linq_proc_macro::*;

#[test]
fn test_select() {
    rql!(select * from user);
}
