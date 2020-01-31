package decorators

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
	"user-service/config"
	"user-service/models"
)

//APIFunc is ...
// type APIFunc func(*gin.Context)

//AuthWrap is auth decorator
func AuthWrap(fn gin.HandlerFunc) gin.HandlerFunc {
	return func(c *gin.Context) {
		token := c.GetHeader("Authorization")
		if token == "" {
			c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
				"message": "Unauthorized",
			})
		}
		user, err := models.VerifyAuthToken(token, config.Conf.DBURL)
		if err != nil {
			log.Println(err)
			c.AbortWithError(401, err)
		} else {
			c.Set("User", user)
			fn(c)
		}
	}
}
