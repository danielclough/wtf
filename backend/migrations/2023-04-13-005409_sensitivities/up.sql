CREATE TABLE sensitivities (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  links Text[] NOT NULL,
  precautions Text[] NOT NULL,
  imparing BOOL NOT NULL,
  life_threatening BOOL NOT NULL,
  dietary BOOL NOT NULL,
  environmental BOOL NOT NULL,
  social BOOL NOT NULL
)
