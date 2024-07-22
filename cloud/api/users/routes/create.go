package usersRoutes

import (
	usersDatabase "envy-api/users/database"

	"github.com/gofiber/fiber/v2"
)

func CreateUser(c *fiber.Ctx) error {
	user, err := usersDatabase.CreateUser()
	if err != nil {
		return c.Status(500).JSON(&fiber.Map{
			"success": false,
			"error":   err.Error(),
		})
	}

	return c.Status(201).JSON(&fiber.Map{
		"success": true,
		"user":    user,
	})
}
