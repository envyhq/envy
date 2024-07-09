package projectsRoutes

import "github.com/gofiber/fiber/v2"

func GetProject(c *fiber.Ctx) error {
	return c.Status(200).JSON(&fiber.Map{
		"todo": true,
	})
}
