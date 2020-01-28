package apiv1

import (
	"log"
	"net/http"
	"user-service/models"

	"github.com/gin-gonic/gin"
)

type RegisterUserParams struct {
	Username string `json:"username"`
	Password string `json:"password"`
	Email    string `json:"email"`
}

func RegisterUser(c *gin.Context) {
	var params RegisterUserParams
	err := c.BindJSON(&params)
	if err != nil {
		log.Println(err)
		c.AbortWithError(http.StatusBadRequest, err)
		return
	}

	var user models.User

	models.DB.Where("name = ?", params.Username).First(&user)
	if user != (models.User{}) {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Username Exists"})
	}
	models.DB.Where("email = ?", params.Email).First(&user)
	if user != (models.User{}) {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Email Exists"})
	}

	user = models.User{
		Name:  params.Username,
		Email: params.Email,
	}
	user.Password([]byte(params.Password))
	db := models.DB.Create(&user)
	if db.Error != nil {
		c.JSON(http.StatusInternalServerError, db.Error)
	}
	c.JSON(http.StatusCreated, gin.H{"message": "Success"})
}
