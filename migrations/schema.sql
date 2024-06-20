CREATE TABLE shirts (
    secret UUID PRIMARY KEY,
    id VARCHAR NOT NULL
);

CREATE INDEX idx_shirt_secret ON shirts (secret);
