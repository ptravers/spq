package main

import (
	"fmt"
	"spq_server/http_server"
)

func main() {
	fmt.Println("Booting")

	// Host needs to be 0.0.0.0 to be accessible in docker for mac
	// https://github.com/mozilla-services/aws-signing-proxy/issues/2
	config := http_server.Config{Host: "0.0.0.0", Port: 8080}
	ports := http_server.Ports{}

	http_server.StartServer(config, ports)
}
