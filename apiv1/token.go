package apiv1

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"

	"user-service/config"
	"user-service/models"
)

func VerifyAuthToken(c *gin.Context) {
	userInterface, _ := c.Get("User")

	user := userInterface.(*models.User)
	c.JSON(http.StatusOK, gin.H{
		"id": user.ID,
	})
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
		fmt.Fprintln(gin.DefaultWriter, params)
		c.AbortWithError(http.StatusBadRequest, err)
		return
	}
	if params.Password == "" {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{
			"message": "Invalid password",
		})
		return
	}

	var user models.User
	if params.Username != "" {
		models.DB.Where("name = ?", params.Username).First(&user)
	} else if params.Email != "" {
		models.DB.Where("name = ?", params.Email).First(&user)
	} else {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Invalid username/password"})
		return
	}
	if ok := user.VerifyPassword(params.Password); ok != true {
		c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{
			"message": "Invalid username/password",
		})
		return
	}

	token, err := user.GenerateAuthToken(config.Conf.SecretKey, config.Conf.ExpiresAt)
	if err != nil {
		c.AbortWithError(http.StatusInternalServerError, err)
	} else {
		c.JSON(http.StatusOK, gin.H{"token": token})
	}
}
