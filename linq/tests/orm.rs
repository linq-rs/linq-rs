use linq_rs::*;

mod utils;
use utils::*;

#[table]
struct User {
    #[column("id_")]
    #[primary(autoinc)]
    id: i32,
    first_name: String,
    last_name: String,
    #[one_to_many(from=col_id to=col_user_id)]
    cards: Card,
    created_time: DateTime,
    updated_time: DateTime,
}

#[table]
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
        id: 1usize.into(),
        first_name: "hello".into(),
        last_name: "world".into(),
        ..Default::default()
    };

    assert_eq!(user.id.value, Some(1));

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
