-- users table
CREATE TABLE users (
    username TEXT PRIMARY KEY NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE api_tokens (
    token UUID PRIMARY KEY NOT NULL,
    user_name TEXT NOT NULL,

    FOREIGN KEY (user_name) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE pastes (
    id UUID PRIMARY KEY NOT NULL,
    title TEXT,
    lang TEXT NOT NULL,
    content TEXT NOT NULL,
    author_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY (author_name) REFERENCES users(username) ON DELETE CASCADE
);

INSERT INTO users VALUES ('Anonymous', ' ');