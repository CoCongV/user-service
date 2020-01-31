package apihandlers

import (
	"net/http"
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
		c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
			"message": "Unauthorized",
		})
	}
	user, err := models.VerifyAuthToken(token, config.Conf.SecretKey)
	if err != nil {
		c.AbortWithError(401, err)
	} else {
		c.Set("User", user)
	}
}
