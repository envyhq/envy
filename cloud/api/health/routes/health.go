package healthRoutes

import "github.com/gofiber/fiber/v2"

func CheckHealth(c *fiber.Ctx) error {
	return c.Status(201).JSON(&fiber.Map{
		"healthy": true,
	})
}
