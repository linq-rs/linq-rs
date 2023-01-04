use linq_rs::*;

mod utils;
#[allow(unused)]
use utils::*;

use serde::{Deserialize, Serialize};

#[derive(ORM, Serialize, Deserialize, Default)]
#[table_name("user_table")]
struct User {
    #[column("id_")]
    #[primary(autoinc)]
    id: Option<i32>,
    first_name: String,
    last_name: String,
    #[cascade(from=col_id to=col_user_id)]
    cards: Vec<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_time: Option<DateTime>,
    updated_time: Option<DateTime>,
}

#[derive(ORM, Serialize, Deserialize, Default)]
struct Card {
    #[primary]
    id: usize,
    user_id: usize,
    card_no: String,
}

#[async_std::test]
async fn test_crud() -> anyhow::Result<()> {
    _ = pretty_env_logger::try_init();

    let mut driver = InsertDriver::default();

    #[allow(unused)]
    let mut user = User {
        id: Some(1),
        first_name: "hello".into(),
        last_name: "world".into(),
        cards: vec![Card::default()].into(),
        ..Default::default()
    };

    user.insert().exec(&mut driver).await?;

    // check generated `INSERT` qir code
    assert_eq!(
        driver.inserter[0],
        rql! {
            INSERT INTO user_table(first_name,last_name)
        }
    );

    let mut driver = SelectDriver::default();

    Vec::<User>::select().exec(&mut driver).await?;

    Ok(())
}
