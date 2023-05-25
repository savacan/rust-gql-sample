-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS buildings (
        `id` BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY,
        `user_id` BIGINT NOT NULL,
        `address` VARCHAR(255) NOT NULL,
        `building_code` BIGINT NOT NULL,
        `rent` BIGINT NOT NULL,
        `minimum_rent` BIGINT DEFAULT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        KEY `fk_buildings_users` (`user_id`),
        CONSTRAINT `fk_buildings_users` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE RESTRICT ON UPDATE CASCADE
    );