table "organisations" {
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
