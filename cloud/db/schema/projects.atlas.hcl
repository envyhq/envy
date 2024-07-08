table "projects" {
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

  column "organisation_id" {
    type = uuid
    null = true
  }
  foreign_key "organisation_fk" {
    columns     = [column.organisation_id]
    ref_columns = [table.organisations.column.id]
  }

  column "created_at" {
    type = date
  }

  column "updated_at" {
    type = date
    null = true
  }

  column "archived_at" {
    type = date
    null = true
  }
}
