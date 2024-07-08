table "organisation_members" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "is_owner" {
    type = boolean
  }

  column "user_id" {
    type = uuid
  }
  foreign_key "user_fk" {
    columns     = [column.user_id]
    ref_columns = [table.users.column.id]
  }

  column "organisation_id" {
    type = uuid
  }
  foreign_key "organisation_fk" {
    columns     = [column.organisation_id]
    ref_columns = [table.organisations.column.id]
  }

  column "created_at" {
    type = date
  }
}
