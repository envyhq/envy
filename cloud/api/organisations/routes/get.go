package organisationsRoutes

import "github.com/gofiber/fiber/v2"

func GetOrganisation(c *fiber.Ctx) error {
	return c.Status(200).JSON(&fiber.Map{
		"todo": true,
	})
}
