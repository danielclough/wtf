CREATE TABLE users (
  id UUID PRIMARY KEY,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  address Text[] NOT NULL,
  address_verified BOOLEAN[] NOT NULL,
  email Text[] NOT NULL UNIQUE,
  email_verified BOOLEAN[] NOT NULL,
  phone Text[] NOT NULL UNIQUE,
  phone_verified BOOLEAN[] NOT NULL,
  taint VARCHAR NOT NULL,
  login_ids UUID[] NOT NULL
)
