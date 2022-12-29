use linq_rs::{orm::*, *};

mod utils;
use utils::*;

#[table]
struct User {
    #[primary]
    id: usize,
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

#[allow(unused)]
#[async_std::test]
async fn test_select_one() -> anyhow::Result<()> {
    let a = 1;

    let order_by = "name".to_string();

    let qir = rql! {
        SELECT id,first_name,last_name,created_time,updated_time FROM User WHERE id = #a ORDER BY #order_by.as_str() DESC LIMIT 1 OFFSET 20
    };

    let mut driver = AssertDriver::exepct_selecter(qir, || vec![User::default()]);

    let row = User::select()
        .cond(rql_where!(id = #a))
        .order_by(&order_by, true)
        .offset(20)
        .exec(&mut driver)
        .await?;

    Ok(())
}

// #[allow(unused)]
// #[async_std::test]
// async fn test_select_many() -> anyhow::Result<()> {
//     let mut driver = NullDriver {};
//     let a = 1;

//     let order_by = "name".to_string();

//     let row = Vec::<NullTable>::select()
//         .cond(rql_where!(id = #a))
//         .order_by(&order_by, true)
//         .limit(100)
//         .offset(20)
//         .exec(&mut driver)
//         .await?;

//     Ok(())
// }
