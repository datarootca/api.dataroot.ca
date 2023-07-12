CREATE TABLE "state" (
    "stateid" uuid NOT NULL,
    "name" varchar NOT NULL UNIQUE,
    "symbol" varchar(2) NOT NULL UNIQUE,
    "created_at" timestamptz NOT NULL DEFAULT now(),
    "updated_at" timestamptz,
    "extid" varchar NOT NULL UNIQUE,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    PRIMARY KEY ("stateid")
);
-- Column Comment
COMMENT ON COLUMN "state"."stateid" IS 'Primary key';
COMMENT ON COLUMN "state"."name" IS 'State name';
COMMENT ON COLUMN "state"."symbol" IS 'State symbol';
COMMENT ON COLUMN "state"."created_at" IS 'Creation timestamp';
COMMENT ON COLUMN "state"."updated_at" IS 'Last update timestamp';
COMMENT ON COLUMN "state"."extid" IS 'External identifier';
COMMENT ON COLUMN "state"."highres_link" IS 'High-res photo link';
COMMENT ON COLUMN "state"."photo_link" IS 'Photo link';
COMMENT ON COLUMN "state"."thumb_link" IS 'Thumbnail link';

INSERT INTO "state" ("stateid", "name", "symbol", "created_at", "updated_at", "extid") VALUES
('d300b6c2-842a-41a8-bde2-8e7367ec28e1', 'Alberta', 'ab', '2023-06-18 19:02:21.564749', NULL, 'ab'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e2', 'British Columbia', 'bc', '2023-06-18 19:02:21.564749', NULL, 'bc'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e3', 'Manitoba', 'mb', '2023-06-18 19:02:21.564749', NULL, 'mb'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e4', 'New Brunswick', 'nb', '2023-06-18 19:03:22.196395', NULL, 'nb'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e5', 'Newfoundland and Labrador', 'nl', '2023-06-18 19:03:22.196395', NULL, 'nl'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e6', 'Nova Scotia', 'ns', '2023-06-18 19:03:22.196395', NULL, 'ns'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e7', 'Ontario', 'on', '2023-06-18 19:03:22.196395', NULL, 'on'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e8', 'Prince Edward Island', 'pe', '2023-06-18 19:03:22.196395', NULL, 'pe'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e9', 'Quebec', 'qc', '2023-06-18 19:04:48.553503', NULL, 'qc'),
('d300b6c2-842a-41a8-bde2-8e7367ec2810', 'Saskatchewan', 'sk', '2023-06-18 19:05:10.051338', NULL, 'sk');