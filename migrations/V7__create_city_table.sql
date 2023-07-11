CREATE SEQUENCE IF NOT EXISTS cityid_cityid_seq;

CREATE TABLE IF NOT EXISTS "city" (
    "cityid" uuid NOT NULL,
    "created_at" timestamptz default now(),
    "updated_at" timestamptz,
    "extid" varchar NOT NULL,
    "name" varchar NOT NULL,
    "slug" varchar NOT NULL,
    "stateid" uuid NOT NULL,
    "highres_link" varchar,
    "photo_link" varchar,
    "thumb_link" varchar,
    PRIMARY KEY ("cityid")
);


INSERT INTO "city" ("cityid", "created_at", "updated_at", "extid", "name", "slug", "stateid") VALUES
('d300b6c2-842a-41a8-bde2-8e7367ec28e1', '2023-06-18 19:07:21.914266', NULL, '1', 'Airdrie', 'airdrie', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e2', '2023-06-18 19:07:21.914266', NULL, '2', 'Cochrane', 'cochrane', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e3', '2023-06-18 19:07:21.914266', NULL, '3', 'Edmonton', 'edmonton', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e4', '2023-06-18 19:07:21.914266', NULL, '4', 'Leduc', 'leduc', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e5', '2023-06-18 19:07:21.914266', NULL, '5', 'Grande Prairie', 'grande-prairie', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e6', '2023-06-18 19:07:21.914266', NULL, '6', 'Red Deer
', 'red-deer', 'd300b6c2-842a-41a8-bde2-8e7367ec28e1'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e7', '2023-06-18 19:07:21.914266', NULL, '7', 'Vancouver', 'vancouver', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e8', '2023-06-18 19:07:21.914266', NULL, '8', 'Victoria', 'victoria', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec28e9', '2023-06-18 19:07:21.914266', NULL, '9', 'Chilliwack', 'chilliwack', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec2810', '2023-06-18 19:07:21.914266', NULL, '10', 'Penticton
', 'penticton
', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec2811', '2023-06-18 19:07:21.914266', NULL, '11', 'Surrrey', 'surrrey', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec2812', '2023-06-18 19:07:21.914266', NULL, '12', 'Prince Rupert', 'prince-rupert', 'd300b6c2-842a-41a8-bde2-8e7367ec28e2'),
('d300b6c2-842a-41a8-bde2-8e7367ec2813', '2023-06-18 19:07:21.914266', NULL, '13', 'Brandon', 'brandon', 'd300b6c2-842a-41a8-bde2-8e7367ec28e3'),
('d300b6c2-842a-41a8-bde2-8e7367ec2814', '2023-06-18 19:07:21.914266', NULL, '14', 'Steinbach', 'steinbach', 'd300b6c2-842a-41a8-bde2-8e7367ec28e3'),
('d300b6c2-842a-41a8-bde2-8e7367ec2815', '2023-06-18 19:07:21.914266', NULL, '15', 'Winnipeg', 'winnipeg', 'd300b6c2-842a-41a8-bde2-8e7367ec28e3'),
('d300b6c2-842a-41a8-bde2-8e7367ec2816', '2023-06-18 19:47:10.6576', NULL, '16', 'Fredericton', 'fredericton', 'd300b6c2-842a-41a8-bde2-8e7367ec28e4'),
('d300b6c2-842a-41a8-bde2-8e7367ec2817', '2023-06-18 19:47:10.6576', NULL, '17', 'Moncton', 'moncton', 'd300b6c2-842a-41a8-bde2-8e7367ec28e4'),
('d300b6c2-842a-41a8-bde2-8e7367ec2818', '2023-06-18 19:47:10.6576', NULL, '18', 'Saint John', 'saint-john', 'd300b6c2-842a-41a8-bde2-8e7367ec28e4'),
('d300b6c2-842a-41a8-bde2-8e7367ec2819', '2023-06-18 19:47:10.6576', NULL, '19', 'Mount Pearl', 'mount-pearl', 'd300b6c2-842a-41a8-bde2-8e7367ec28e5'),
('d300b6c2-842a-41a8-bde2-8e7367ec2820', '2023-06-18 19:47:10.6576', NULL, '20', 'St. John''s', 'st-johns', 'd300b6c2-842a-41a8-bde2-8e7367ec28e5'),
('d300b6c2-842a-41a8-bde2-8e7367ec2821', '2023-06-18 19:47:10.6576', NULL, '21', 'Halifax', 'halifax', 'd300b6c2-842a-41a8-bde2-8e7367ec28e6'),
('d300b6c2-842a-41a8-bde2-8e7367ec2822', '2023-06-18 19:47:10.6576', NULL, '22', 'Sydney', 'sydney', 'd300b6c2-842a-41a8-bde2-8e7367ec28e6'),
('d300b6c2-842a-41a8-bde2-8e7367ec2823', '2023-06-18 19:47:10.6576', NULL, '23', 'Dartmouth', 'dartmouth', 'd300b6c2-842a-41a8-bde2-8e7367ec28e6'),
('d300b6c2-842a-41a8-bde2-8e7367ec2824', '2023-06-18 19:47:10.6576', NULL, '24', 'Corner Brook', 'corner-brook', 'd300b6c2-842a-41a8-bde2-8e7367ec28e5'),
('d300b6c2-842a-41a8-bde2-8e7367ec2825', '2023-06-18 19:47:10.6576', NULL, '25', 'Toronto', 'toronto', 'd300b6c2-842a-41a8-bde2-8e7367ec28e7'),
('d300b6c2-842a-41a8-bde2-8e7367ec2826', '2023-06-18 19:47:10.6576', NULL, '26', 'Ottawa
', 'ottawa
', 'd300b6c2-842a-41a8-bde2-8e7367ec28e7'),
('d300b6c2-842a-41a8-bde2-8e7367ec2827', '2023-06-18 19:47:10.6576', NULL, '27', 'Mississauga', 'mississauga', 'd300b6c2-842a-41a8-bde2-8e7367ec28e7'),
('d300b6c2-842a-41a8-bde2-8e7367ec2828', '2023-06-18 19:47:10.6576', NULL, '28', 'Charlottetown', 'charlottetown', 'd300b6c2-842a-41a8-bde2-8e7367ec28e8'),
('d300b6c2-842a-41a8-bde2-8e7367ec2829', '2023-06-18 19:47:10.6576', NULL, '29', 'Summerside', 'summerside', 'd300b6c2-842a-41a8-bde2-8e7367ec28e8'),
('d300b6c2-842a-41a8-bde2-8e7367ec2830', '2023-06-18 19:47:10.6576', NULL, '30', 'Stratford', 'stratford', 'd300b6c2-842a-41a8-bde2-8e7367ec28e8'),
('d300b6c2-842a-41a8-bde2-8e7367ec2831', '2023-06-18 19:47:10.6576', NULL, '31', 'Montreal', 'montreal', 'd300b6c2-842a-41a8-bde2-8e7367ec28e9'),
('d300b6c2-842a-41a8-bde2-8e7367ec2832', '2023-06-18 19:47:10.6576', NULL, '32', 'Quebec City', 'quebec-city', 'd300b6c2-842a-41a8-bde2-8e7367ec28e9'),
('d300b6c2-842a-41a8-bde2-8e7367ec2833', '2023-06-18 19:47:10.6576', NULL, '33', 'Laval', 'laval', 'd300b6c2-842a-41a8-bde2-8e7367ec28e9'),
('d300b6c2-842a-41a8-bde2-8e7367ec2834', '2023-06-18 19:07:21.914266', NULL, '34', 'Prince Albert', 'prince-albert', 'd300b6c2-842a-41a8-bde2-8e7367ec2810'),
('d300b6c2-842a-41a8-bde2-8e7367ec2835', '2023-06-18 19:07:21.914266', NULL, '35', 'Saskatoon', 'saskatoon', 'd300b6c2-842a-41a8-bde2-8e7367ec2810'),
('d300b6c2-842a-41a8-bde2-8e7367ec2836', '2023-06-18 19:07:21.914266', NULL, '36', 'Regina', 'regina', 'd300b6c2-842a-41a8-bde2-8e7367ec2810');