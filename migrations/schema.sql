CREATE TABLE shirts (
    id VARCHAR PRIMARY KEY,
    secret UUID NOT NULL,
    redirect_url VARCHAR NOT NULL,
    UNIQUE(secret)
);

CREATE INDEX idx_shirt_id ON shirts (id);
CREATE INDEX idx_shirt_secret ON shirts (secret);
