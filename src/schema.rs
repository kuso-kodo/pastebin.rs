table! {
    api_tokens (token) {
        token -> Uuid,
        user_name -> Text,
    }
}

table! {
    pastes (id) {
        id -> Uuid,
        title -> Nullable<Text>,
        lang -> Text,
        content -> Text,
        author_name -> Text,
        created_at -> Timestamp,
    }
}

table! {
    users (username) {
        username -> Text,
        password -> Text,
    }
}

joinable!(api_tokens -> users (user_name));
joinable!(pastes -> users (author_name));

allow_tables_to_appear_in_same_query!(api_tokens, pastes, users,);
