CREATE SEQUENCE IF NOT EXISTS organizer_organizerid_seq;

CREATE TABLE "organizer" (
    "organizerid" int4 NOT NULL DEFAULT nextval('organizer_organizerid_seq'::regclass),
    "firstname" varchar,
    "lastname" varchar,
    "created_at" timestamptz default now(),
    "updated_at" timestamptz,
    "extid" varchar,
    "image" varchar,
    "bio" varchar,
    "bg" varchar,
    PRIMARY KEY ("organizerid")
);