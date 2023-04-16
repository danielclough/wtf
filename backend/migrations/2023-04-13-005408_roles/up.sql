CREATE TABLE roles (
  id UUID PRIMARY KEY,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  responsibility VARCHAR NOT NULL,
  discount VARCHAR NOT NULL,
  seen_by_role Uuid[] NOT NULL
)
