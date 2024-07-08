table "environments" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "name" {
    type = varchar(255)
  }

  column "project_id" {
    type = uuid
  }
  foreign_key "project_fk" {
    columns     = [column.project_id]
    ref_columns = [table.projects.column.id]
  }

  column "created_at" {
    type = date
  }

  column "updated_at" {
    type = date
    null = true
  }
}
