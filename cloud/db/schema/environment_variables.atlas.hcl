table "environment_variables" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "environment_id" {
    type = uuid
  }
  foreign_key "environment_fk" {
    columns     = [column.environment_id]
    ref_columns = [table.environments.column.id]
  }

  column "variable_id" {
    type = uuid
  }
  foreign_key "variable_fk" {
    columns     = [column.variable_id]
    ref_columns = [table.variables.column.id]
  }

  column "created_at" {
    type = date
  }
}
