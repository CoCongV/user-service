package config

import (
	"log"

	"github.com/BurntSushi/toml"
)

//Config struct
type Config struct {
	Addr      string `toml:"addr"`
	DBURL     string `toml:"dbURL"`
	SecretKey string `toml:"secret_key"`
	ExpiresAt int64  `toml:"expires_at"`
}

//Conf is struct Config point
var Conf *Config

//Setup is read toml config file for init
func Setup() {
	filepath := "./config.toml"
	_, err := toml.DecodeFile(filepath, &Conf)
	if err != nil {
		log.Fatal(err)
	}
}
