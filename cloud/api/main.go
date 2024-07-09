package main

import (
	authenticationRoutes "envy-api/authentication/routes"
	healthRoutes "envy-api/health/routes"
	organisationsRoutes "envy-api/organisations/routes"
	projectsRoutes "envy-api/projects/routes"
	usersRoutes "envy-api/users/routes"
	"log"

	"github.com/gofiber/fiber/v2"
)

func main() {
	app := fiber.New()

	app.Get("/health", healthRoutes.CheckHealth)

	app.Post("/sign-in", authenticationRoutes.SignIn)
	app.Post("/sign-out", authenticationRoutes.SignOut)
	app.Post("/reset-password", authenticationRoutes.ResetPassword)
	app.Post("/verify-email", authenticationRoutes.VerifyEmail)

	app.Post("/users", usersRoutes.CreateUser)
	app.Get("/me", usersRoutes.GetMe)
	app.Patch("/me", usersRoutes.PatchMe)

	app.Post("/organisations", organisationsRoutes.CreateOrganisation)
	app.Get("/organisations/:id", organisationsRoutes.GetOrganisation)
	app.Patch("/organisations/:id", organisationsRoutes.PatchOrganisation)

	app.Post("/projects", projectsRoutes.CreateProject)
	app.Get("/projects/:id", projectsRoutes.GetProject)
	app.Patch("/projects/:id", projectsRoutes.PatchProject)

	log.Fatal(app.Listen(":4000"))
}
