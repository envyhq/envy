package organisationsRoutes

import "github.com/gofiber/fiber/v2"

func GetOrganisation(c *fiber.Ctx) error {
	// TODO: fetch from db
	return c.Status(200).JSON(&fiber.Map{
		"todo": true,
	})
}
