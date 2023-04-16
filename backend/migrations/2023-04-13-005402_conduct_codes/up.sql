CREATE TABLE conduct_codes (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  qualifications Text[] NOT NULL,
  restrictions Text[] NOT NULL,
  examples Text[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
