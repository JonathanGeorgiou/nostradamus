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

CREATE TABLE IF NOT EXISTS "team" (
  "id" serial PRIMARY KEY,
  "full_name" varchar,
  "short_name" varchar,
  "three_letter_name" varchar
);

INSERT INTO TEAM (full_name, short_name, three_letter_name) VALUES
('Arsenal', 'Arsenal', 'ARS'), 
('Aston Villa', 'Villa', 'AVL'),
('Bournemouth', 'Bournemouth', 'BOU'),
('Brentford', 'Brentford', 'BRE'),
('Brighton and Hove Albion', 'Brighton', 'BRI'),
('Chelsea', 'Chelsea', 'CHL'),
('Crystal Palace', 'Palace', 'CRY'),
('Everton', 'Everton', 'EVE'),
('Fulham', 'Fulham', 'FUL'),
('Leeds United', 'Leeds', 'LEE'),
('Leicester City', 'Leicester', 'LEI'),
('Liverpool', 'Liverpool', 'LIV'),
('Manchester City', 'Man City', 'MCY'),
('Manchester United', 'Man United', 'MUN'),
('Newcastle United', 'Newcastle', 'NEW'),
('Nottingham Forest', 'Forest', 'NOT'),
('Southampton', 'Southampton', 'SOU'),
('Tottenham Hotspur', 'Spurs', 'TOT'),
('West Ham United', 'West Ham', 'WHU'),
('Wolverhampton Wanderers', 'Wolves', 'WOL');

COMMENT ON COLUMN "fixture"."result" IS '0: home win, 1: away win, 2: draw';

ALTER TABLE "player" ADD FOREIGN KEY ("id") REFERENCES "prediction" ("player_id");

ALTER TABLE "prediction" ADD FOREIGN KEY ("fixture_id") REFERENCES "fixture" ("id");

