table! {
    api_tokens (token) {
        token -> Uuid,
        user_id -> Uuid,
    }
}

table! {
    pastes (id) {
        id -> Uuid,
        title -> Nullable<Text>,
        content -> Text,
        author_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
    }
}

joinable!(api_tokens -> users (user_id));
joinable!(pastes -> users (author_id));

allow_tables_to_appear_in_same_query!(
    api_tokens,
    pastes,
    users,
);
