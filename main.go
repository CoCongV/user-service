package main

import (
	"log"
	"os"

	"github.com/gin-gonic/gin"
	_ "github.com/jinzhu/gorm/dialects/postgres"
	"github.com/urfave/cli/v2"

	"user-service/apiv1"
	"user-service/config"
	"user-service/models"
	"user-service/server"
)

var confPath string
var confFlag = &cli.StringFlag{
	Name:        "conf",
	Value:       "",
	Usage:       "config file path",
	Destination: &confPath,
}

func init() {
	config.Setup()
	models.Setup()
}

func main() {
	app := &cli.App{
		Commands: []*cli.Command{
			{
				Name:   "runserver",
				Usage:  "run server",
				Action: runserver,
			},
			{
				Name:   "migrate",
				Usage:  "migrate models",
				Action: migrate,
			},
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

func runserver(c *cli.Context) error {

	defer models.DB.Close()

	r := server.CreateServ()

	r.Use(gin.Recovery())

	apiv1.SetRouter(r)
	r.Run(config.Conf.Addr)
	return nil
}

func migrate(c *cli.Context) error {
	models.DB.AutoMigrate(&models.User{})

	return nil
}
