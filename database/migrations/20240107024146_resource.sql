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
CREATE TABLE "anime_movie" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "anime_tv_show" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "manga" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "light_novel" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "video_game" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "board_game" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
CREATE TABLE "visual_novel" (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL UNIQUE REFERENCES resource (id) ON DELETE CASCADE
);
