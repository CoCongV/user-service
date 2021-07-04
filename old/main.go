package main

import (
	"log"
	"os"

	"net/http"
	_ "net/http/pprof"

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
			{
				Name:   "init",
				Usage:  "migrate and create admin user",
				Action: initServer,
			},
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

func runserver(c *cli.Context) error {
	go func() {
		log.Println(http.ListenAndServe("localhost:6060", nil))
	}()

	defer models.DB.Close()

	r := server.CreateServ()

	apiv1.SetRouter(r)
	r.Run(config.Conf.Addr)
	return nil
}

func migrate(c *cli.Context) error {
	models.DB.AutoMigrate(&models.User{})

	return nil
}

func initServer(c *cli.Context) error {
	migrate(c)
	user := models.User{
		Name:  config.Conf.AdminUser,
		Email: config.Conf.AdminEmail,
	}
	user.Password([]byte(config.Conf.AdminPasswd))
	models.DB.Create(&user)
	return nil
}
