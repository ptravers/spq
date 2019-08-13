package http_server

import (
	"github.com/matryer/is"
	"net/http"
	"net/http/httptest"
	"testing"
)

var testConfig = Config{Host: "localhost", Port: 8080}
var testPorts = Ports{}

func TestHealthCheck(t *testing.T) {
	is := is.New(t)
	srv := server{
		config: testConfig,
		ports:  testPorts,
	}

	req, err := http.NewRequest("GET", "/health", nil)
	is.NoErr(err)
	w := httptest.NewRecorder()
	srv.healthHandler(w, req)

	is.Equal(w.Result().StatusCode, http.StatusOK)
}

func TestNextItemStatus(t *testing.T) {
	is := is.New(t)
	srv := server{
		config: testConfig,
		ports:  testPorts,
	}

	req, err := http.NewRequest("GET", "/items/next", nil)
	is.NoErr(err)
	w := httptest.NewRecorder()
	srv.healthHandler(w, req)

	is.Equal(w.Result().StatusCode, http.StatusOK)
}

