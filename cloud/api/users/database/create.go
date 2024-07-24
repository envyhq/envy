package usersDatabase

import (
	database "envy-api/database"
	"fmt"
	"time"

	"github.com/google/uuid"
)

type User struct {
	CreatedAt time.Time `json:"created_at" db:"created_at"`
	ID        string    `json:"id" db:"id"`
	FirstName string    `json:"first_name" db:"first_name"`
	LastName  string    `json:"last_name" db:"last_name"`
	Email     string    `json:"email" db:"email"`
	Password  string    `json:"password" db:"password"`
	Language  string    `json:"language" db:"language"`
	Currency  string    `json:"currency" db:"currency"`
}

func CreateUser() (User, error) {
	db, err := database.Connect()
	if err != nil {
		return User{}, err
	}

	// TODO: take from req
	user := User{
		ID:        uuid.New().String(),
		FirstName: "John",
		LastName:  "Doe",
		Email:     "john.doe@envy.io",
		Password:  "password",
		Language:  "en",
		Currency:  "usd",
		CreatedAt: time.Now(),
	}

	fmt.Println("Creating user", user)

	tx := db.MustBegin()
	_, err = tx.NamedExec(`INSERT INTO users (
    id, first_name, last_name, email, password, language, currency, created_at
  ) VALUES (
    :id, :first_name, :last_name, :email, :password, :language, :currency, :created_at
  )`,
		&user,
	)
	if err != nil {
		return User{}, err
	}

	err = tx.Commit()
	if err != nil {
		return User{}, err
	}

	fmt.Println("User created", user)

	return user, nil
}
