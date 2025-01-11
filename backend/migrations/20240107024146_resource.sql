-- Add migration script here
CREATE TABLE "resource" (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  year_published INTEGER,
  thumbnail TEXT,
  api_id INTEGER NOT NULL UNIQUE
);
CREATE TABLE "tag" (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);
CREATE TABLE "tagging" (
  id SERIAL PRIMARY KEY,
  tag_id INTEGER NOT NULL REFERENCES tag (id),
  resource_id INTEGER NOT NULL REFERENCES resource (id),
  UNIQUE (tag_id, resource_id)
);
