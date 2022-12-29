use linq_rs::{
    ddl::{Column, Constraint, Create, NamedConstraint, DDL},
    *,
};

#[test]
fn test_create_table() {
    let table_name = "User";

    let qirs = ddl! {
        CREATE TABLE User(
            id INT PRIMARY,
            name STRING,
            date DATETIME,
            CONSTRAINT name_index UNIQUE(name),
            CONSTRAINT date_index INDEX(date),
        );

        CREATE TABLE Card(
            id INT PRIMARY AUTOINC,
            user_id INT,
            date DATETIME,
            CONSTRAINT user_id_foreign_key FOREIGN KEY (user_id) REFERENCES User(id),
        );

        // alter column
        ALTER TABLE Card ALTER COLUMN user_id BIGINT;

        ALTER TABLE Card ADD COLUMN card_no INT;

        ALTER TABLE Card RENAME COLUMN card_no TO no;

        ALTER TABLE Card DROP COLUMN no;

        // alter constraint

        ALTER TABLE Card ADD CONSTRAINT date_index INDEX(date);

        ALTER TABLE Card ALTER CONSTRAINT date_index UNIQUE(date);

        ALTER TABLE Card RENAME CONSTRAINT date_index TO date_unique;

        ALTER TABLE Card DROP CONSTRAINT date_unique;

        DROP TABLE Card;

        TRUNCATE TABLE #table_name;
    };

    assert_eq!(
        qirs[0],
        DDL::Create(Create {
            table_name: "User",
            cols: vec![
                Column {
                    name: "id",
                    col_type: IrType::Int,
                    not_null: false,
                    default_value: None,
                    primary: Some(false),
                },
                Column {
                    name: "name",
                    col_type: IrType::String,
                    not_null: false,
                    default_value: None,
                    primary: None,
                },
                Column {
                    name: "date",
                    col_type: IrType::DateTime,
                    not_null: false,
                    default_value: None,
                    primary: None,
                }
            ],
            constraints: vec![
                NamedConstraint {
                    name: "name_index",
                    constraint: Constraint::Unique(vec!["name"])
                },
                NamedConstraint {
                    name: "date_index",
                    constraint: Constraint::Index(vec!["date"])
                }
            ]
        })
    );

    assert_eq!(
        qirs[1],
        DDL::Create(Create {
            table_name: "Card",
            cols: vec![
                Column {
                    name: "id",
                    col_type: IrType::Int,
                    not_null: false,
                    default_value: None,
                    primary: Some(true),
                },
                Column {
                    name: "user_id",
                    col_type: IrType::Int,
                    not_null: false,
                    default_value: None,
                    primary: None,
                },
                Column {
                    name: "date",
                    col_type: IrType::DateTime,
                    not_null: false,
                    default_value: None,
                    primary: None,
                }
            ],
            constraints: vec![NamedConstraint {
                name: "user_id_foreign_key",
                constraint: Constraint::ForeignKey(vec!["user_id"], "User", vec!["id"])
            },]
        })
    );
}
