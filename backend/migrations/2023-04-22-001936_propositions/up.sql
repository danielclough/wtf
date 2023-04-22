CREATE TABLE propositions (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  credence REAL NOT NULL,
  description Text[] NOT NULL,
  links Text[] NOT NULL,
  qualifications Text[] NOT NULL,
  restrictions Text[] NOT NULL
)
