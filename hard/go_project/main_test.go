package main

import (
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
)

func TestHelloHandler(t *testing.T) {
	tests := []struct {
		name           string
		queryParam     string
		expectedBody   string
		expectedStatus int
	}{
		{
			name:           "без параметра name",
			queryParam:     "",
			expectedBody:   "Hello, World!",
			expectedStatus: http.StatusOK,
		},
		{
			name:           "с параметром name",
			queryParam:     "name=Alice",
			expectedBody:   "Hello, Alice!",
			expectedStatus: http.StatusOK,
		},
		{
			name:           "с пустым параметром name",
			queryParam:     "name=",
			expectedBody:   "Hello, World!",
			expectedStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req, err := http.NewRequest("GET", "/?"+tt.queryParam, nil)
			if err != nil {
				t.Fatal(err)
			}

			rr := httptest.NewRecorder()
			handler := http.HandlerFunc(helloHandler)
			handler.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.expectedStatus {
				t.Errorf("handler returned wrong status code: got %v want %v",
					status, tt.expectedStatus)
			}

			body := strings.TrimSpace(rr.Body.String())
			if body != tt.expectedBody {
				t.Errorf("handler returned unexpected body: got %v want %v",
					body, tt.expectedBody)
			}
		})
	}
}

func TestHealthHandler(t *testing.T) {
	req, err := http.NewRequest("GET", "/health", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(healthHandler)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	expected := `{"status": "healthy", "service": "go-app"}`
	if rr.Body.String() != expected {
		t.Errorf("handler returned unexpected body: got %v want %v",
			rr.Body.String(), expected)
	}

	contentType := rr.Header().Get("Content-Type")
	if contentType != "application/json" {
		t.Errorf("handler returned wrong content type: got %v want %v",
			contentType, "application/json")
	}
}

func TestVersionHandler(t *testing.T) {
	req, err := http.NewRequest("GET", "/version", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(versionHandler)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	expected := `{"version": "1.0.0", "build": "static"}`
	if rr.Body.String() != expected {
		t.Errorf("handler returned unexpected body: got %v want %v",
			rr.Body.String(), expected)
	}

	contentType := rr.Header().Get("Content-Type")
	if contentType != "application/json" {
		t.Errorf("handler returned wrong content type: got %v want %v",
			contentType, "application/json")
	}
}

func TestGetEnv(t *testing.T) {
	// Сохраняем оригинальное значение переменной окружения
	originalValue := os.Getenv("TEST_VAR")
	defer os.Setenv("TEST_VAR", originalValue)

	// Тест 1: переменная не установлена, возвращается значение по умолчанию
	os.Unsetenv("TEST_VAR")
	result := getEnv("TEST_VAR", "default")
	if result != "default" {
		t.Errorf("getEnv() = %v, want %v", result, "default")
	}

	// Тест 2: переменная установлена, возвращается её значение
	os.Setenv("TEST_VAR", "custom")
	result = getEnv("TEST_VAR", "default")
	if result != "custom" {
		t.Errorf("getEnv() = %v, want %v", result, "custom")
	}
}