use linq_rs::*;

#[table]
struct User {
    #[column("id_")]
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
