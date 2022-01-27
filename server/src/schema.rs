table! {
    todo (id_todo) {
        id_todo -> Integer,
        owner -> Text,
        title -> Text,
        date -> Text,
        priority -> Integer,
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
