package main

import (
	"fmt"
	"spq_server/http_server"
  "spq_server/ports"
)

type Config struct {
  Host string
  Port int
  ServerType string
}

func main() {
	fmt.Println("Booting")

  file, err := os.Open(filename) if err != nil {  panic(err) }

  decoder := json.NewDecoder(file)

  var config Config

  err = decoder.Decode(&config)
  if err != nil {  panic(err) }

  switch config.ServerType {
  case "GRPC":


  case "HTTP":
	  config := HttpConfig{Host: config.Host, Port: config.Port}
	  ports := ports.Ports{}

	  http_server.StartServer(config, ports)
  case default:
    panic(errors.New("Invalid service type supported types are GRPC and HTTP"))
  }
}
