CREATE TABLE `user`(
	`id`            INTEGER NOT NULL    PRIMARY KEY AUTOINCREMENT,
    `name`  TEXT    NOT NULL,
    `contact_id`    INTEGER NOT NULL,
    `created_by_id` INTEGER NOT NULL,
    FOREIGN KEY ('contact_id') REFERENCES 'contact'('id'),
    FOREIGN KEY ('created_by_id') REFERENCES 'user'('id')
);

CREATE TABLE `contact`(
    `id`    INTEGER NOT NULL    PRIMARY KEY AUTOINCREMENT,
    `name`  TEXT    NOT NULL
);


INSERT INTO "contact" (id, name) VALUES (1, "Lukas");
INSERT INTO "contact" (id, name) VALUES (2, "Dennis");
INSERT INTO "contact" (id, name) VALUES (3, "Tom");
INSERT INTO "contact" (id, name) VALUES (4, "Mareike");

INSERT INTO "user" (id, contact_id, name, created_by_id) VALUES (1, 1, "Admin", 1);
INSERT INTO "user" (id, contact_id, name, created_by_id) VALUES (2, 1, "Lukas-User", 1);
INSERT INTO "user" (id, contact_id, name, created_by_id) VALUES (3, 2, "Dennis-User", 1);
INSERT INTO "user" (id, contact_id, name, created_by_id) VALUES (4, 3, "Tom-User",2);
INSERT INTO "user" (id, contact_id, name, created_by_id) VALUES (5, 4, "Mareike-User",2);

PRAGMA foreign_keys = ON;
