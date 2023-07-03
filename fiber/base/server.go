package main

import "github.com/gofiber/fiber/v2"

func main() {
    app := fiber.New(fiber.Config{
        AppName:"Application",
        Prefork:true,
    })

    app.Get("/", func(c *fiber.Ctx) error {
        return c.SendString("Hello world!")
    })

    app.Get("/users", func(c *fiber.Ctx) error {
        return c.SendString("List of users")
    })

    app.Listen(":6060")
}
