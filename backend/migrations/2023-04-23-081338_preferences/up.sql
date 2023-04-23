CREATE TABLE preferences (
  id UUID PRIMARY KEY,
  browser_theme VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  pronouns VARCHAR NOT NULL,
  role_ids UUID[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
