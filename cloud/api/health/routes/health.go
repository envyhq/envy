package healthRoutes

import "github.com/gofiber/fiber/v2"

func CheckHealth(c *fiber.Ctx) error {
	// TODO: check db connection
	// TODO: check ui connection
	return c.Status(201).JSON(&fiber.Map{
		"api": true,
		"ui":  "todo",
		"db":  "todo",
	})
}
