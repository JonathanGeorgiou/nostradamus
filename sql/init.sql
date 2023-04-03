CREATE TABLE IF NOT EXISTS "player" (
  "id" serial PRIMARY KEY,
  "username" varchar,
  "first_name" varchar,
  "last_name" varchar,
  "email" varchar,
  "is_active" bool,
  "points" integer,
  "created_at" timestamp
);

CREATE TABLE IF NOT EXISTS "fixture" (
  "id" serial PRIMARY KEY,
  "home_team" varchar,
  "away_team" varchar,
  "home_score" integer,
  "away_score" integer,
  "result" integer
);

CREATE TABLE IF NOT EXISTS "prediction" (
  "id" serial PRIMARY KEY,
  "fixture_id" integer,
  "player_id" integer,
  "home_score" integer,
  "away_score" integer,
  "result" integer
);

COMMENT ON COLUMN "fixture"."result" IS '0: home win, 1: away win, 2: draw';

ALTER TABLE "player" ADD FOREIGN KEY ("id") REFERENCES "prediction" ("player_id");

ALTER TABLE "prediction" ADD FOREIGN KEY ("fixture_id") REFERENCES "fixture" ("id");

