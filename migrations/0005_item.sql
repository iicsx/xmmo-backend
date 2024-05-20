CREATE TABLE IF NOT EXISTS "usage_group" (
 group_name     VARCHAR(50) PRIMARY KEY NOT NULL,
 "limit"        INT NOT NULL
);

CREATE TABLE IF NOT EXISTS "item" (
  id            INT PRIMARY KEY   NOT NULL,
  name          VARCHAR(250)      NOT NULL,
  itype         VARCHAR(250)      NOT NULL,
  rarity        VARCHAR(250)      NOT NULL,
  weight        DECIMAL(10, 2)    NOT NULL,
  img           VARCHAR(250)      NOT NULL,
  group_name    VARCHAR(50)       REFERENCES "usage_group"(group_name),
  description   TEXT,
  stat_type     VARCHAR(250),
  stat_value    INT,
  mod_type      VARCHAR(250),
  mod_value     INT,
  mod_duration  INT,
  req_type      VARCHAR(250),
  req_value     INT               
);

CREATE TABLE IF NOT EXISTS "item_usage" (
  item_id       INT REFERENCES "item"(id),
  user_id       INT REFERENCES "user"(id),
  usages        INT NOT NULL,
  PRIMARY KEY   (item_id, user_id)
);

CREATE INDEX IF NOT EXISTS item_id_idx on item(id);
