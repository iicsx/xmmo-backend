CREATE TABLE IF NOT EXISTS "refresh_token" (
    user_id     INT PRIMARY KEY  NOT NULL,
    token       TEXT             NOT NULL
);

CREATE INDEX IF NOT EXISTS user_id ON "refresh_token" (user_id);
