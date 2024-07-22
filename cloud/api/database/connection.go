package database

import (
	"github.com/jmoiron/sqlx"
	_ "github.com/tursodatabase/go-libsql"
)

func Connect() (*sqlx.DB, error) {
	db, err := sqlx.Connect("libsql", "libsql://envy-development-lbennett-stacki.turso.io?authToken=eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJpYXQiOjE3MjA0ODY0MDcsImlkIjoiODgwNzQzMzMtODc5Yi00OWRmLTgzODEtOWNkMmViMWNlNjgwIn0.X1MKTci4zdVWlOBtwcsWUpH0YUhWbNifheJG16Bc6ZUVhJZMfNKnH2sk8jWubU8I1V3oSnYfbaDs_li8LvmFDQ")

	return db, err
}
