package organisationsRoutes

import "github.com/gofiber/fiber/v2"

func PatchOrganisation(c *fiber.Ctx) error {
	// TODO: fetch from db
	return c.Status(201).JSON(&fiber.Map{
		"todo": true,
	})
}
