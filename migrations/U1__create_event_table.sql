CREATE SEQUENCE IF NOT EXISTS event_eventid_seq;

CREATE TABLE IF NOT EXISTS "event" (
    "eventid" int4 NOT NULL DEFAULT nextval('event_eventid_seq'::regclass),
    "name" varchar,
    "description" varchar,
    "created_at" timestamptz default now(),
    "updated_at" timestamptz,
    "extid" varchar,
    "image" varchar,
    "location" varchar NOT NULL,
    "groupid" int4 NOT NULL,
    "status" varchar NOT NULL,
    "in_person" bool NOT NULL,
    "time" timestamp NOT NULL,
    "duration" int4,
    "link" varchar,
    "waitlist_count" int4,
    "is_online" bool,
    "yes_rsvp_count" int4,
    "fee" bool,
    PRIMARY KEY ("eventid")
);