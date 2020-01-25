package apiv1

import (
	"github.com/gin-gonic/gin"
	"log"

	"user-service/models"
)

type VerifyAuthTokenReq struct {
	Token string `json:"token" binding:"required"`
}

func VerifyAuthToken(c *gin.Context) {
	var params VerifyAuthTokenReq
	err := c.BindJSON(&params)
	if err != nil {
		log.Println(err)
		c.AbortWithError(400, err)
		return
	}
	user, err := models.VerifyAuthToken(params.Token, "123")
	if err != nil {
		log.Panicln(err)
		c.AbortWithError(401, err)
	} else {
		c.JSON(200, gin.H{"id": user.ID})
	}
}
