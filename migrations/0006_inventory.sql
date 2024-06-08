CREATE TABLE IF NOT EXISTS "inventory" (
  user_id       INT REFERENCES "user"(id)    PRIMARY KEY,
  quantity      INT DEFAULT 1,
  level         INT DEFAULT 0,
  item_id       INT REFERENCES "item"(id)    NOT NULL,
  name          VARCHAR(250)                 NOT NULL,
  itype         VARCHAR(250)                 NOT NULL,
  rarity        VARCHAR(250)                 NOT NULL,
  weight        DECIMAL(10, 2)               NOT NULL,
  img           VARCHAR(250)                 NOT NULL,
);

CREATE INDEX IF NOT EXISTS inventory_user_id_idx ON inventory(user_id);
