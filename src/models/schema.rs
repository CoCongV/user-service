table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        avatar -> Text,
        verify -> Bool,
        password_hash -> Text,
        role -> Integer,
    }
}