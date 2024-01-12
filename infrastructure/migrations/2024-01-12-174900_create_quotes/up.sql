CREATE TABLE quotes (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR NOT NULL,
    quote VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);
