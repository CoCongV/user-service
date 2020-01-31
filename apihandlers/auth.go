package apihandlers

import (
	"user-service/config"
	"user-service/models"

	"github.com/gin-gonic/gin"
)

//APIFunc is ...
// type APIFunc func(*gin.Context)

//AuthHandler is auth decorator
func AuthHandler(c *gin.Context) {
	token := c.GetHeader("Authorization")
	if token == "" {
		c.AbortWithStatus(401)
		return
	}
	user, err := models.VerifyAuthToken(token, config.Conf.SecretKey)
	if err != nil {
		c.AbortWithError(401, err)
	} else {
		c.Set("User", user)
	}
}
