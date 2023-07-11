CREATE TABLE IF NOT EXISTS "article" (
    "articleid" uuid NOT NULL,
    "extid" varchar,
    "name" varchar,
    "description" varchar,
    "time_m" varchar,
    "publish_at" timestamptz,
    "source" varchar,
    "link" varchar,
    "author" varchar,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    "created_at" timestamptz default now(),
    "updated_at" timestamptz,
    PRIMARY KEY ("articleid")
);