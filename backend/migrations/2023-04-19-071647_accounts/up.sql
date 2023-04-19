CREATE TABLE accounts (
  id UUID PRIMARY KEY,
  avatar VARCHAR NOT NULL,
  level VARCHAR NOT NULL,
  preference_id UUID NOT NULL,
  role_ids UUID[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL,
  survey_results_id UUID NOT NULL
)
