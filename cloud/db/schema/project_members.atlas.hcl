table "project_members" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "user_id" {
    type = uuid
  }
  foreign_key "user_fk" {
    columns     = [column.user_id]
    ref_columns = [table.users.column.id]
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
}
