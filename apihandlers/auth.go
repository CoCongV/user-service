package apihandlers

import (
	"log"
	"strings"
	"user-service/config"
	"user-service/models"

	"github.com/gin-gonic/gin"
)

//APIFunc is ...
// type APIFunc func(*gin.Context)

//AuthHandler is auth decorator
func AuthHandler(c *gin.Context) {
	auth := c.GetHeader("Authorization")
	ss := strings.Split(auth, " ")
	token := ss[len(ss)-1]
	if token == "" {
		c.AbortWithStatus(401)
		return
	}
	user, err := models.VerifyAuthToken(token, config.Conf.SecretKey)
	if err != nil {
		log.Println(token)
		c.AbortWithError(401, err)
	} else {
		c.Set("User", user)
	}
}
