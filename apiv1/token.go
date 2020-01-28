package apiv1

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"

	"user-service/config"
	"user-service/models"
)

func VerifyAuthToken(c *gin.Context) {
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
		c.JSON(200, gin.H{"id": user.ID})
	}
}

type GenerateAuthTokenParams struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func GenerateAuthToken(c *gin.Context) {
	var params GenerateAuthTokenParams
	err := c.BindJSON(&params)
	if err != nil {
		log.Println(&params)
		c.AbortWithError(http.StatusUnauthorized, err)
		return
	}

	var user models.User
	if params.Username != "" {
		models.DB.Where("name = ?", params.Username).First(&user)
	} else if params.Email != "" {
		models.DB.Where("name = ?", params.Email).First(&user)
	} else {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Wrong User name or Password"})
	}
	token, err := user.GenerateAuthToken(config.Conf.SecretKey, config.Conf.ExpiresAt)
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
	} else {
		c.JSON(http.StatusOK, gin.H{"token": token})
	}
}
