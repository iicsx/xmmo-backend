CREATE TABLE IF NOT EXISTS "item" (
  id              INT PRIMARY KEY     NOT NULL,
  name            VARCHAR(250)        NOT NULL,
  itype           VARCHAR(250)        NOT NULL,
  rarity          VARCHAR(250)        NOT NULL,
  stat_type       VARCHAR(250)        NOT NULL,
  stat_value      INT                 NOT NULL,
  mod_type        VARCHAR(250)        NOT NULL,
  mod_value       INT                 NOT NULL,
  mod_duration    INT                 NOT NULL,
  req_type        VARCHAR(250)        NOT NULL,
  req_value       INT                 NOT NULL,
  iweight         DECIMAL(10, 2)      NOT NULL,
  img             VARCHAR(250)        NOT NULL,
  idesc           TEXT                NOT NULL
);

CREATE INDEX IF NOT EXISTS item_id_idx on item(id);
