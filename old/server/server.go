package server

import (
	"github.com/gin-gonic/gin"
	"user-service/config"
)

//CreateServ return gin engine
func CreateServ() *gin.Engine {
	r := gin.New()
	r.Use(gin.Recovery())
	if config.Conf.Mode == "debug" {
		r.Use(gin.Logger())
	}
	return r
}
