-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS users (
        `id` BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        `name` VARCHAR(255) NOT NULL,
        `role` INT(11) NOT NULL
    );