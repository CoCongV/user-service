package models

import "fmt"

type User struct {
	ID     uint   `gorm:"primary_key"`
	Name   string `gorm:"unique_index;not null;type:varchar(32)"`
	Avatar string `gorm:"not null;size:255"`
}

func (u User) String() string {
	return fmt.Sprintf("User<%d %s>", u.ID, u.Name)
}
