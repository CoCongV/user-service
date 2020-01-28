package main

import (
	"bytes"
	"encoding/json"
	"log"
	"net/http"
	"net/http/httptest"
	"testing"
	"user-service/apiv1"
	"user-service/models"
	"user-service/server"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
)

//TestSuit struct
type TestSuit struct {
	suite.Suite
	server *gin.Engine
	w      *httptest.ResponseRecorder
}

//SetupTest init test suit
func (suit *TestSuit) SetupTest() {
	suit.server = server.CreateServ()
	apiv1.SetRouter(suit.server)
	models.DB = models.InitDB("host=127.0.0.1 port=5432 user=cong dbname=userservice password=password sslmode=disable")
	suit.w = httptest.NewRecorder()
}

func (suit *TestSuit) TearDownSuite() {
	models.DB.Unscoped().Where("name = ?", "test1").Delete(models.User{})
}

func (suit *TestSuit) TestUser() {
	params := apiv1.RegisterUserParams{
		Username: "test1",
		Email:    "test1@test.com",
		Password: "password",
	}
	paramsBytes, err := json.Marshal(params)
	if err != nil {
		log.Fatal(err)
	}

	req, _ := http.NewRequest("POST", "/api/v1/users", bytes.NewBuffer(paramsBytes))
	suit.server.ServeHTTP(suit.w, req)
	assert.Equal(suit.T(), 201, suit.w.Code)
}

func TestUserTestSuit(t *testing.T) {
	suite.Run(t, new(TestSuit))
}
