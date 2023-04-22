CREATE TABLE events (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL,
  description Text[] NOT NULL,
  imgs Text[] NOT NULL,
  links Text[] NOT NULL,
  ticketing Text[] NOT NULL,
  location Text[] NOT NULL,
  directions Text[] NOT NULL,
  map_images Text[] NOT NULL,
  start_time VARCHAR NOT NULL,
  end_time VARCHAR NOT NULL,
  conduct_code_ids UUID[] NOT NULL,
  other_expectations Text[] NOT NULL,
  account_ids UUID[] NOT NULL,
  sensitivity_ids UUID[] NOT NULL
)
