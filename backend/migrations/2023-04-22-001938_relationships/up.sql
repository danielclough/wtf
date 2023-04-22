CREATE TABLE relationships (
  id UUID PRIMARY KEY,
  dog_cat_bird VARCHAR NOT NULL,
  ignore_ids UUID[] NOT NULL,
  friend_ids UUID[] NOT NULL,
  frienenmy_ids UUID[] NOT NULL,
  neutral_ids UUID[] NOT NULL
)
