CREATE TABLE accounts (
  id UUID PRIMARY KEY,
  avatar VARCHAR NOT NULL,
  level VARCHAR NOT NULL,
  preference_ids UUID[] NOT NULL,
  relationship_ids UUID[] NOT NULL,
  survey_results_ids UUID[] NOT NULL,
  user_ids UUID[] NOT NULL
)
