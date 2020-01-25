package apiv1

import (
	"github.com/gin-gonic/gin"
	"log"
)

type VerifyAuthTokenReq struct {
	Token string `json:"token" binding:"required"`
}

func VerifyAuthToken(c *gin.Context) {
	var params VerifyAuthTokenReq
	err := c.BindJSON(&params)
	if err != nil {
		log.Println(err)
		c.JSON(400, gin.H{"message": "Bad Request"})
		return
	}
}
