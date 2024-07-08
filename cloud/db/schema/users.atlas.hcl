table "users" {
  schema = schema.main

  column "id" {
    type = uuid
  }
  primary_key {
    columns = [
      column.id
    ]
  }

  column "first_name" {
    type = varchar(255)
  }

  column "last_name" {
    type = varchar(255)
  }

  column "email" {
    type = varchar(255)
  }

  column "password" {
    type = varchar(255)
  }

  column "language" {
    type = varchar(255)
  }

  column "currency" {
    type = varchar(255)
  }

  column "created_at" {
    type = date
  }

  column "updated_at" {
    type = date
    null = true
  }
}
