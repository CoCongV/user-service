package models

import (
	"fmt"

	"github.com/dgrijalva/jwt-go"
	"golang.org/x/crypto/bcrypt"
)

const (
	UPLOAD = 8
	ADMIN  = 16
)

//User is model
type User struct {
	ID           uint   `gorm:"primary_key"`
	Name         string `gorm:"unique_index;not null;type:varchar(32)"`
	Email        string `gorm:"unique_index;type:varchar(64)"`
	Avatar       string `gorm:"not null;size:255"`
	Verify       string `gorm:"type:BOOLEAN;default:false"`
	PasswordHash string `gorm:"type:varchar(256);not null"`
	Role         uint   `gorm:"not null"`
}

//CustomClaims is custom jwt claims
type CustomClaims struct {
	ID uint `json:"id"`
	jwt.StandardClaims
}

func (u User) String() string {
	return fmt.Sprintf("User<%d %s>", u.ID, u.Name)
}

//GenerateAuthToken is ...
func (u *User) GenerateAuthToken(secretKey string, expiresAt int64) (string, error) {
	claims := CustomClaims{
		u.ID,
		jwt.StandardClaims{
			ExpiresAt: expiresAt,
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString([]byte(secretKey))
}

//VerifyAuthToken verify login token
func VerifyAuthToken(tokenString, secretKey string) (*User, error) {
	var claims CustomClaims
	_, err := jwt.ParseWithClaims(tokenString, &claims, func(token *jwt.Token) (interface{}, error) {
		return []byte(secretKey), nil
	})
	if err != nil {
		return &User{}, err
	}

	var user User
	DB.First(&user, claims.ID)
	return &user, nil
}

//Password hash password and store
func (u *User) Password(password []byte) error {
	hash, err := bcrypt.GenerateFromPassword(password, bcrypt.DefaultCost)
	if err != nil {
		return err
	}
	u.PasswordHash = string(hash)
	return nil
}

//VerifyPassword is ...
func (u *User) VerifyPassword(password string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(u.PasswordHash), []byte(password))
	if err != nil {
		return false
	}
	return true
}
