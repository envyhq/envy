package authenticationRoutes

import "github.com/gofiber/fiber/v2"

func SignIn(c *fiber.Ctx) error {
	// TODO: fetch from db
	return c.Status(200).JSON(&fiber.Map{
		"todo": true,
	})
}
