package main

import (
	"bytes"
	"encoding/json"
	"io/ioutil"
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
func (suit *TestSuit) SetupSuite() {
	suit.server = server.CreateServ()
	apiv1.SetRouter(suit.server)
	createUser()
}

func (suit *TestSuit) TearDownSuite() {
	delUser()
}

func (suit *TestSuit) TestUser() {
	w := httptest.NewRecorder()
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
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 201, w.Code)
}

func (suit *TestSuit) TestToken() {
	w := httptest.NewRecorder()
	params := apiv1.GenerateAuthTokenParams{
		Username: "test",
		Password: "password",
	}
	paramsBytes, err := json.Marshal(params)
	if err != nil {
		log.Fatal(err)
	}
	req, _ := http.NewRequest("POST", "/api/v1/generate_auth_token", bytes.NewBuffer(paramsBytes))
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 200, w.Code)
	resp := w.Result()
	body, _ := ioutil.ReadAll(resp.Body)

	var result struct {
		Token string `json:"token"`
	}
	if err := json.Unmarshal(body, &result); err != nil {
		log.Fatalln(err)
	}

	req, _ = http.NewRequest("GET", "/api/v1/verify_auth_token", nil)
	req.Header.Set("Authorization", result.Token)
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 200, w.Code)
}

func (suit *TestSuit) TestUnauth() {
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/api/v1/verify_auth_token", nil)
	req.Header.Set("Authorization", "testest")
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 401, w.Code)

	req, _ = http.NewRequest("GET", "/api/v1/verify_auth_token", nil)
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 401, w.Code)

}

func (suit *TestSuit) TestBadReq() {
	w := httptest.NewRecorder()
	params := make(map[string]string)
	params["username"] = "test1"
	paramsBytes, err := json.Marshal(params)
	if err != nil {
		log.Fatal(err)
	}
	req, _ := http.NewRequest("POST", "/api/v1/generate_auth_token", bytes.NewBuffer(paramsBytes))
	suit.server.ServeHTTP(w, req)
	assert.Equal(suit.T(), 400, w.Code)
}

func (suit *TestSuit) TestUserModel() {
	var user models.User
	models.DB.Where("name = ?", "test").First(&user)
	assert.Equal(suit.T(), "test", user.Name)
	assert.Equal(suit.T(), "test@test.com", user.Email)
	assert.Equal(suit.T(), true, user.VerifyPassword("password"))
}

func TestUserTestSuit(t *testing.T) {
	suite.Run(t, new(TestSuit))
}

func createUser() {
	user := models.User{
		Name:  "test",
		Email: "test@test.com",
	}
	user.Password([]byte("password"))
	models.DB.Create(&user)
}

func delUser() {
	var user models.User
	models.DB.Where("name = ?", "test").First(&user)
	models.DB.Delete(&user)
	models.DB.Unscoped().Where("name = ?", "test1").Delete(models.User{})
}
