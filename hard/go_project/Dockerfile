# Многостадийная сборка
# Первая стадия: сборка статического бинарника
FROM golang:1.23-alpine AS builder

WORKDIR /app

# Копируем исходный код
COPY go.mod go.sum ./
RUN go mod download

COPY . .

# Собираем статический бинарник
RUN CGO_ENABLED=0 go build -a -installsuffix cgo -ldflags="-s -w -extldflags '-static'" -o go-app main.go

# Вторая стадия: scratch образ
FROM scratch

# Копируем бинарник из builder
COPY --from=builder /app/go-app /go-app

# Копируем сертификаты CA (опционально, для HTTPS запросов)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Указываем порт
EXPOSE 8000

# Запускаем приложение
ENTRYPOINT ["/go-app"]