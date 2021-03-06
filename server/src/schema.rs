table! {
    pref (id) {
        id -> Integer,
        id_owner -> Integer,
        sort -> Integer,
        display -> Integer,
    }
}

table! {
    todo (id) {
        id -> Integer,
        progress -> Integer,
        id_owner -> Integer,
        title -> Text,
        date -> Text,
        priority -> Integer,
        content -> Text,
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

allow_tables_to_appear_in_same_query!(pref, todo, user,);
