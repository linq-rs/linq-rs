use linq_rs::*;

mod utils;
#[allow(unused)]
use utils::*;

use serde::{Deserialize, Serialize};

#[table]
#[derive(Serialize, Deserialize)]
struct User {
    #[column("id_")]
    #[primary(autoinc)]
    id: Option<i32>,
    first_name: String,
    last_name: String,
    #[cascade(from=col_id to=col_user_id)]
    cards: Vec<Card>,
    created_time: Option<DateTime>,
    updated_time: Option<DateTime>,
}

#[table]
#[derive(Serialize, Deserialize)]
struct Card {
    #[primary]
    id: usize,
    user_id: usize,
    card_no: String,
}

#[async_std::test]
async fn test_crud() -> anyhow::Result<()> {
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
        driver.inserter,
        Some(rql! {
            INSERT INTO User(first_name,last_name)
        })
    );

    Ok(())
}
