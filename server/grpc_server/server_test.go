package grpc_server

import (
	"github.com/matryer/is"
  "testing"
  "spq_server/ports"
)

var testConfig = Config{Host: "localhost", Port: 8080}
var testPorts = ports.Ports{}

func TestHealthCheck(t *testing.T) {
  
}
