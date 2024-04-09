CREATE TABLE IF NOT EXISTS user_stats (
  user_id         INT REFERENCES "user"(id) PRIMARY KEY,
  ledges_grabbed  INT NOT NULL DEFAULT 0,
  npc_kills       INT NOT NULL DEFAULT 0,
  items_dropped   INT NOT NULL DEFAULT 0,
  height          DECIMAL(10, 2) NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS user_details (
  user_id           INT REFERENCES "user"(id) PRIMARY KEY,
  strength          INT NOT NULL DEFAULT 0,
  defence           INT NOT NULL DEFAULT 0,  
  dexterity         INT NOT NULL DEFAULT 0,
  current_energy    INT NOT NULL DEFAULT 5,
  max_energy        INT NOT NULL DEFAULT 5,
  current_hp        INT NOT NULL DEFAULT 10,
  max_hp            INT NOT NULL DEFAULT 10,
  exp               INT NOT NULL DEFAULT 0,
  profession        INT,
  profession_exp    INT
);

CREATE INDEX IF NOT EXISTS user_stats_user_id_idx ON user_stats(user_id);
CREATE INDEX IF NOT EXISTS user_details_user_id_idx ON user_details(user_id);
