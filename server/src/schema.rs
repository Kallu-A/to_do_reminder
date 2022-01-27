table! {
    todo (id) {
        id -> Integer,
        owner -> Integer,
        title -> Text,
        date -> Text,
        importance -> Integer,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        perm -> Bool,
        picture -> Bool,
        email -> Text,
        confirm_email -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(todo, user,);
