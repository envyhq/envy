package usersRoutes

import "github.com/gofiber/fiber/v2"

func GetMe(c *fiber.Ctx) error {
	return c.Status(200).JSON(&fiber.Map{
		"todo": true,
	})
}
