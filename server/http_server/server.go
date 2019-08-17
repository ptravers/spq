package http_server

import (
	"fmt"
	"net/http"
)

type Config struct {
	Host string
	Port int
}

type Ports struct {}

type server struct {
	ports  Ports
	config Config
}

func (s *server) healthHandler(w http.ResponseWriter, r *http.Request) {
	w.WriteHeader(http.StatusOK)
}

func (s *server) nextItemHandler(w http.ResponseWriter, r *http.Request) {
  w.WriteHeader(http.StatusOK)
}

func (s *server) attachRoutes() {
	http.HandleFunc("/health", s.healthHandler)
  http.HandleFunc("/items/next", s.nextItemHandler)
}

func (s *server) start() {
	err := http.ListenAndServe(fmt.Sprintf("%s:%d", s.config.Host, s.config.Port), nil)
	if err != nil {
		panic(err)
	}
}

func StartServer(config Config, ports Ports) {
	server := server{
		ports,
		config,
	}

	server.attachRoutes()

	server.start()
}
