CREATE TABLE logins (
  id UUID PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  pw_salt VARCHAR NOT NULL,
  pw_hash VARCHAR NOT NULL,
  mfa_salt VARCHAR NOT NULL,
  mfa_hash VARCHAR NOT NULL
)
