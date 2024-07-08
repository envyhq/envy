-- Add column "is_owner" to table: "project_members"
ALTER TABLE `project_members` ADD COLUMN `is_owner` boolean NOT NULL;
