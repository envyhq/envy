-- Create "environment_variables" table
CREATE TABLE `environment_variables` (`id` uuid NOT NULL, `environment_id` uuid NOT NULL, `variable_id` uuid NOT NULL, `created_at` date NOT NULL, PRIMARY KEY (`id`), CONSTRAINT `environment_fk` FOREIGN KEY (`environment_id`) REFERENCES `environments` (`id`), CONSTRAINT `variable_fk` FOREIGN KEY (`variable_id`) REFERENCES `variables` (`id`));
-- Create "environments" table
CREATE TABLE `environments` (`id` uuid NOT NULL, `name` varchar NOT NULL, `project_id` uuid NOT NULL, `created_at` date NOT NULL, `updated_at` date NULL, PRIMARY KEY (`id`), CONSTRAINT `project_fk` FOREIGN KEY (`project_id`) REFERENCES `projects` (`id`));
-- Create "organisation_members" table
CREATE TABLE `organisation_members` (`id` uuid NOT NULL, `is_owner` bool NOT NULL, `user_id` uuid NOT NULL, `organisation_id` uuid NOT NULL, `created_at` date NOT NULL, PRIMARY KEY (`id`), CONSTRAINT `user_fk` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `organisation_fk` FOREIGN KEY (`organisation_id`) REFERENCES `organisations` (`id`));
-- Create "organisations" table
CREATE TABLE `organisations` (`id` uuid NOT NULL, `name` varchar NOT NULL, `created_at` date NOT NULL, `updated_at` date NULL, PRIMARY KEY (`id`));
-- Create "project_members" table
CREATE TABLE `project_members` (`id` uuid NOT NULL, `user_id` uuid NOT NULL, `project_id` uuid NOT NULL, `created_at` date NOT NULL, PRIMARY KEY (`id`), CONSTRAINT `user_fk` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`), CONSTRAINT `project_fk` FOREIGN KEY (`project_id`) REFERENCES `projects` (`id`));
-- Create "projects" table
CREATE TABLE `projects` (`id` uuid NOT NULL, `name` varchar NOT NULL, `organisation_id` uuid NULL, `created_at` date NOT NULL, `updated_at` date NULL, PRIMARY KEY (`id`), CONSTRAINT `organisation_fk` FOREIGN KEY (`organisation_id`) REFERENCES `organisations` (`id`));
-- Create "users" table
CREATE TABLE `users` (`id` uuid NOT NULL, `first_name` varchar NOT NULL, `last_name` varchar NOT NULL, `email` varchar NOT NULL, `password` varchar NOT NULL, `language` varchar NOT NULL, `currency` varchar NOT NULL, `created_at` date NOT NULL, `updated_at` date NULL, PRIMARY KEY (`id`));
-- Create "variable_values" table
CREATE TABLE `variable_values` (`id` uuid NOT NULL, `value` varchar NOT NULL, `variable_id` uuid NOT NULL, `created_at` date NOT NULL, PRIMARY KEY (`id`), CONSTRAINT `variable_fk` FOREIGN KEY (`variable_id`) REFERENCES `variables` (`id`));
-- Create "variables" table
CREATE TABLE `variables` (`id` uuid NOT NULL, `key` varchar NOT NULL, `description` varchar NOT NULL, `is_private` bool NOT NULL, `project_id` uuid NOT NULL, `created_at` date NOT NULL, `updated_at` date NULL, PRIMARY KEY (`id`), CONSTRAINT `project_fk` FOREIGN KEY (`project_id`) REFERENCES `projects` (`id`));
