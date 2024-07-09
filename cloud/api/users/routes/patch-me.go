package usersRoutes

import "github.com/gofiber/fiber/v2"

func PatchMe(c *fiber.Ctx) error {
	return c.Status(201).JSON(&fiber.Map{
		"todo": true,
	})
}
