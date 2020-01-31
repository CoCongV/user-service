package apiv1

import (
	"github.com/gin-gonic/gin"

	"user-service/apihandlers"
)

// SetRouter init api v1 blueprint
func SetRouter(e *gin.Engine) {
	v1 := e.Group("/api/v1")
	v1.GET("/verify_auth_token", apihandlers.AuthHandler, VerifyAuthToken)
	v1.POST("/generate_auth_token", GenerateAuthToken)
	v1.POST("/users", RegisterUser)
}
