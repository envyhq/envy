package organisationsRoutes

import "github.com/gofiber/fiber/v2"

type Organisation struct {
	ID int `json:"id"`
}

func CreateOrganisation(c *fiber.Ctx) error {
	// TODO: fetch from db
	organisation := CreateOrganisationLib()

	return c.Status(201).JSON(&fiber.Map{
		"success":      true,
		"organisation": organisation,
	})
}

func CreateOrganisationLib() Organisation {
	return Organisation{ID: 1}
}
