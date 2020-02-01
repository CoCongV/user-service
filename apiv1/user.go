package apiv1

import (
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
		c.AbortWithError(http.StatusBadRequest, err)
		return
	}

	var user models.User
	models.DB.Where("name = ?", params.Username).First(&user)
	if user != (models.User{}) {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Username Exists"})
		return
	}
	models.DB.Where("email = ?", params.Email).First(&user)
	if user != (models.User{}) {
		c.AbortWithStatusJSON(http.StatusBadRequest, gin.H{"message": "Email Exists"})
		return
	}

	user = models.User{
		Name:  params.Username,
		Email: params.Email,
	}
	if err := user.Password([]byte(params.Password)); err != nil {
		c.AbortWithError(500, err)
		return
	}
	db := models.DB.Create(&user)
	if db.Error != nil {
		c.AbortWithError(http.StatusInternalServerError, db.Error)
		return
	}
	c.JSON(http.StatusCreated, gin.H{"message": "Success"})
}
