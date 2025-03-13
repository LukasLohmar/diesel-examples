CREATE TABLE `test`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`bool_value` BLOB,
	`int_value` BLOB,
	`float_value` BLOB,
	`string_value` BLOB
);

INSERT INTO "test" (bool_value) VALUES (jsonb_array(true));
INSERT INTO "test" (bool_value) VALUES (jsonb_array(false));
INSERT INTO "test" (int_value) VALUES (jsonb_array(1));
INSERT INTO "test" (float_value) VALUES (jsonb_array(1.5));
INSERT INTO "test" (string_value) VALUES (jsonb_array('string'));
