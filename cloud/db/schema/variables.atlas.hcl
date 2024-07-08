table "variables" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "key" {
    type = varchar(255)
  }

  column "description" {
    type = varchar(255)
  }

  column "is_private" {
    type = bool
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
