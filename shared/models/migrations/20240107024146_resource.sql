-- Add migration script here
CREATE TABLE "resource" (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  year_published INTEGER,
  owned BOOLEAN NOT NULL,
  want_to_own BOOLEAN NOT NULL,
  want_to_try BOOLEAN NOT NULL
  -- thumbnail BYTEA
);
