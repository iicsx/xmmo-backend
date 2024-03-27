CREATE TABLE IF NOT EXISTS permission (
    id          SERIAL PRIMARY KEY  NOT NULL,
    name        VARCHAR(250)        NOT NULL,
    description VARCHAR(250)        NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS user_permission (
  user_id       INT REFERENCES "user"(id) PRIMARY KEY,
  permission_id INT REFERENCES permission(id)
);

DELETE FROM permission;
INSERT INTO permission (name, description) VALUES ('admin', 'Administrators have access to all resources');
INSERT INTO permission (name, description) VALUES ('mod', 'Moderators have access to resources pertaining to their moderation duties');
INSERT INTO permission (name, description) VALUES ('user', 'Users have access to their own resources');
