CREATE TABLE arguments (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  proposition_ids UUID[] NOT NULL,
  relationship VARCHAR NOT NULL
)
