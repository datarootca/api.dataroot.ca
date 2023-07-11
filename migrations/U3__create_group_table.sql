CREATE SEQUENCE IF NOT EXISTS group_groupid_seq;

CREATE TABLE IF NOT EXISTS "group" (
    "groupid" int4 NOT NULL DEFAULT nextval('group_groupid_seq'::regclass),
    "name" varchar,
    "description" varchar,
    "extid" varchar,
    "slug" varchar,
    "active" bool,
    "private" bool,
    "members" int4,
    "cityid" int4,
    "organizer" varchar,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    "created_at" timestamptz default now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("groupid")
);