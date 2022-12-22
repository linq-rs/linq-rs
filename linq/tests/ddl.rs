use linq_rs::*;

#[test]
fn test_create_table() {
    ddl! {
        CREATE TABLE User(
            id INT PRIMARY,
            name STRING,
            date DATETIME,
            CONSTRAINT name_index UNIQUE(name),
            CONSTRAINT date_index INDEX(date),
        );

        CREATE TABLE Card(
            id INT PRIMARY,
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
    };

    let table_name = "User";

    ddl! {
        TRUNCATE TABLE #table_name;
    };
}
