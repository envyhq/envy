-- Add column "archived_at" to table: "organisations"
ALTER TABLE `organisations` ADD COLUMN `archived_at` date NULL;
-- Add column "archived_at" to table: "projects"
ALTER TABLE `projects` ADD COLUMN `archived_at` date NULL;
