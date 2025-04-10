CREATE TABLE `user`(
	`id`            INTEGER NOT NULL    PRIMARY KEY AUTOINCREMENT,
    `contact_id`    INTEGER NOT NULL,
    `created_by`    INTEGER NOT NULL,
    FOREIGN KEY ('contact_id') REFERENCES 'contact'('id')
);

CREATE TABLE `contact`(
    `id`    INTEGER NOT NULL    PRIMARY KEY AUTOINCREMENT,
    `name`  TEXT    NOT NULL
);


INSERT INTO "contact" (id, name) VALUES (1, "Lukas");
INSERT INTO "contact" (id, name) VALUES (2, "Dennis");
INSERT INTO "contact" (id, name) VALUES (3, "Tom");
INSERT INTO "contact" (id, name) VALUES (4, "Mareike");

INSERT INTO "user" (id, contact_id, created_by) VALUES (1, 1, 1);
INSERT INTO "user" (id, contact_id, created_by) VALUES (1, 2, 1);
INSERT INTO "user" (id, contact_id, created_by) VALUES (1, 3, 2);
INSERT INTO "user" (id, contact_id, created_by) VALUES (1, 4, 3);

PRAGMA foreign_keys = ON;
