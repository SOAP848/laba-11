.PHONY: build build-static test clean run docker-build docker-run

# Имя бинарного файла
BINARY_NAME=go-app
VERSION=1.0.0

# Флаги для статической компиляции
LDFLAGS=-ldflags="-s -w -extldflags '-static'"
CGO_ENABLED=0

build:
	go build -o $(BINARY_NAME) main.go

build-static:
	CGO_ENABLED=$(CGO_ENABLED) go build -a -installsuffix cgo $(LDFLAGS) -o $(BINARY_NAME) main.go

test:
	go test -v ./...

clean:
	rm -f $(BINARY_NAME)

run: build
	./$(BINARY_NAME)

docker-build:
	docker build -t $(BINARY_NAME):$(VERSION) -f Dockerfile .

docker-run:
	docker run --rm -p 8080:8080 $(BINARY_NAME):$(VERSION)