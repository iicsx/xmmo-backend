CREATE TABLE IF NOT EXISTS "user" (
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(250)        NOT NULL UNIQUE,
    email       VARCHAR(250)        NOT NULL UNIQUE,
    password    VARCHAR(250)        NOT NULL,
    created_at  TIMESTAMP           NOT NULL DEFAULT NOW(),
    last_login  TIMESTAMP           NOT NULL DEFAULT NOW(),
    muted       BOOLEAN             NOT NULL DEFAULT false,
    locked      BOOLEAN             NOT NULL DEFAULT false,
    banned      BOOLEAN             NOT NULL DEFAULT false
);

CREATE INDEX IF NOT EXISTS user_email_idx ON "user" (email);
