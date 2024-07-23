package projectsRoutes

import "github.com/gofiber/fiber/v2"

type Project struct {
	ID int `json:"id"`
}

func CreateProject(c *fiber.Ctx) error {
	// TODO: fetch from db
	project := CreateProjectLib()

	return c.Status(201).JSON(&fiber.Map{
		"success": true,
		"project": project,
	})
}

func CreateProjectLib() Project {
	return Project{ID: 1}
}
