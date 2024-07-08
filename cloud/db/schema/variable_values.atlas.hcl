table "variable_values" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "value" {
    type = varchar(255)
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
