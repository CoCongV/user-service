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

var Conf *Config

//ReadConfig is read toml config file
func ReadConfig(filePath string) *Config {
	_, err := toml.DecodeFile(filePath, &Conf)
	if err != nil {
		log.Fatal(err)
	}

	return Conf
}
