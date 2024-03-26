-- Up
CREATE TABLE IF NOT EXISTS "user"
(
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    email       VARCHAR(250)        NOT NULL,
    password    VARCHAR(250)        NOT NULL,
    created_at  TIMESTAMP           NOT NULL DEFAULT NOW(),
    last_login  TIMESTAMP           NOT NULL DEFAULT NOW(),
    banned      BOOLEAN             NOT NULL DEFAULT false
);

-- Down
-- DROP TABLE IF EXISTS "user";
