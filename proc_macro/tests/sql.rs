use linq_proc_macro::*;
use linq_rs_ir::dml::SelectColumns;

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
