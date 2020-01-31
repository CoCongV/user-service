package apiHandlers

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
	"user-service/config"
	"user-service/models"
)

//APIFunc is ...
// type APIFunc func(*gin.Context)

//APIAuthHandler is auth decorator
func AuthHandler(c *gin.Context) {
	token := c.GetHeader("Authorization")
	if token == "" {
		c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
			"message": "Unauthorized",
		})
	}
	user, err := models.VerifyAuthToken(token, config.Conf.SecretKey)
	if err != nil {
		log.Println(err)
		c.AbortWithError(401, err)
	} else {
		c.Set("User", user)
	}
}
