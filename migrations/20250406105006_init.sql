CREATE TABLE users
(
    id        UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    name      TEXT        NOT NULL,
    email     TEXT,
    created   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
