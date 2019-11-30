-- users table
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    password TEXT NOT NULL
);

CREATE TABLE api_tokens (
    token UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE pastes (
    id UUID PRIMARY KEY NOT NULL,
    title TEXT,
    author_id UUID,

    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);