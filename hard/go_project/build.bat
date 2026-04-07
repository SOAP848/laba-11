@echo off
set BINARY_NAME=go-app.exe
set VERSION=1.0.0

echo Building static binary...
set CGO_ENABLED=0
go build -a -installsuffix cgo -ldflags="-s -w -extldflags -static" -o %BINARY_NAME% main.go

echo Build completed: %BINARY_NAME%
echo File info:
dir %BINARY_NAME%