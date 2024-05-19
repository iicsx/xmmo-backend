CREATE TABLE IF NOT EXISTS "item" (
  item_id         INT PRIMARY KEY     NOT NULL,
  item_name       VARCHAR(250)        NOT NULL,
  item_type       VARCHAR(250)        NOT NULL,
  item_rarity     VARCHAR(250)        NOT NULL,
  stat_type       VARCHAR(250)        NOT NULL,
  stat_value      INT                 NOT NULL,
  mod_type        VARCHAR(250)        NOT NULL,
  mod_value       INT                 NOT NULL,
  mod_duration    INT                 NOT NULL,
  req_type        VARCHAR(250)        NOT NULL,
  req_value       INT                 NOT NULL,
  item_weight     DECIMAL(10, 2)      NOT NULL,
  item_img        VARCHAR(250)        NOT NULL,
  item_desc       TEXT                NOT NULL
);

CREATE INDEX IF NOT EXISTS item_id_idx on item(item_id);
