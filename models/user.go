package models

import "fmt"

import "github.com/dgrijalva/jwt-go"

import "log"

type User struct {
	ID     uint   `gorm:"primary_key"`
	Name   string `gorm:"unique_index;not null;type:varchar(32)"`
	Avatar string `gorm:"not null;size:255"`
}

type CustomClaims struct {
	ID uint `json:"id"`
	jwt.StandardClaims
}

func (u User) String() string {
	return fmt.Sprintf("User<%d %s>", u.ID, u.Name)
}

func (u *User) GenerateAuthToken(secretKey string, expiresAt int64) (string, error) {
	claims := CustomClaims{
		u.ID,
		jwt.StandardClaims{
			ExpiresAt: expiresAt,
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(secretKey)
}

func (u *User) VerifyAuthToken(tokenString, secretKey string) bool {
	var claims CustomClaims
	_, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		return []byte(secretKey), nil
	})
	if err != nil {
		log.Println(err)
		return false
	}

	if u.ID == claims.ID {
		return true
	} else {
		return false
	}
}
