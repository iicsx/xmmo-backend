CREATE TABLE IF NOT EXISTS permission (
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(250)        NOT NULL UNIQUE,
    description VARCHAR(250)        NOT NULL
);

CREATE TABLE IF NOT EXISTS user_permission (
  user_id       INT REFERENCES "user"(id) PRIMARY KEY,
  permission_id INT REFERENCES permission(id)
);

INSERT INTO permission (name, description) VALUES ('admin', 'Administrators have access to all resources') ON CONFLICT DO NOTHING;
INSERT INTO permission (name, description) VALUES ('mod', 'Moderators have access to resources pertaining to their moderation duties') ON CONFLICT DO NOTHING;
INSERT INTO permission (name, description) VALUES ('user', 'Users have access to their own resources') ON CONFLICT DO NOTHING;
