#!/bin/bash

set -e

BINARY_NAME="go-app"
VERSION="1.0.0"

echo "Building static binary..."
CGO_ENABLED=0 go build -a -installsuffix cgo -ldflags="-s -w -extldflags '-static'" -o "$BINARY_NAME" main.go

echo "Build completed: $BINARY_NAME"
echo "File info:"
file "$BINARY_NAME"
ls -lh "$BINARY_NAME"