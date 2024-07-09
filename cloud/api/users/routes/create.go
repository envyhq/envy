package usersRoutes

import "github.com/gofiber/fiber/v2"

type User struct {
	ID int `json:"id"`
}

func CreateUser(c *fiber.Ctx) error {
	user := CreateUserLib()

	return c.Status(201).JSON(&fiber.Map{
		"success": true,
		"user":    user,
	})
}

func CreateUserLib() User {
	return User{ID: 1}
}
