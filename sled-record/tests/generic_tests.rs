use sled_record::*;

#[derive(Record, Debug, PartialEq)]
struct User {
    id: u64,

    name: String,
    age: u8,
    email: String,
}

fn test_db() -> sled::Db {
    sled::Config::default().temporary(true).open().unwrap()
}

#[test]
fn user_roundtrip() {
    let db = test_db();

    let alice = User {
        id: 420,
        name: "Alice".to_owned(),
        age: 40,
        email: "alice@alice.fr".to_owned(),
    };

    db.persist(&alice).unwrap();

    let (first_user_key, first_user_value) = db
        .open_tree("User")
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .unwrap();

    let rountripped = User::from_kv(&first_user_key, &first_user_value).unwrap();

    assert_eq!(alice, rountripped);
}
