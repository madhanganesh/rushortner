-- urls table
CREATE TABLE IF NOT EXISTS urls (
    id          BIGSERIAL PRIMARY KEY,           -- auto-incrementing ID
    long_url    TEXT NOT NULL,                   -- original URL
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
