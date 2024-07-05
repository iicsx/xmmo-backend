CREATE TABLE IF NOT EXISTS "inventory" (
  user_id       INT NOT NULL REFERENCES "user"(id),
  quantity      INT NOT NULL DEFAULT 1,
  level         INT,
  item_id       INT NOT NULL REFERENCES "item"(id)
);
