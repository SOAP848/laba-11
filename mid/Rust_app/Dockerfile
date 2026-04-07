# Используем официальный образ Rust для сборки (последняя стабильная версия)
FROM rust:1.86 AS builder

# Устанавливаем системные зависимости для сборки
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию
WORKDIR /app

# Копируем файлы манифеста Cargo
COPY Cargo.toml Cargo.lock ./

# Создаем пустой исходный файл для кэширования зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Скачиваем и компилируем зависимости (кэшируем)
RUN cargo build --release

# Копируем реальный исходный код
COPY src ./src

# Пересобираем приложение с реальным кодом
RUN cargo build --release

# Запускаем тесты внутри контейнера сборки
RUN cargo test --release

# Второй этап: создаем минимальный образ для запуска
FROM debian:bookworm-slim

# Устанавливаем системные зависимости для запуска (например, OpenSSL)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Создаем пользователя для безопасности
RUN useradd -m -u 1000 rustuser

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем скомпилированный бинарник из builder
COPY --from=builder /app/target/release/rust_app /app/rust_app

# Меняем владельца файла
RUN chown rustuser:rustuser /app/rust_app

# Переключаемся на непривилегированного пользователя
USER rustuser

# Открываем порт, который будет использовать приложение
EXPOSE 8080

# Устанавливаем переменные окружения
ENV PORT=8080
ENV HOST=0.0.0.0
ENV ENV=production

# Запускаем приложение
CMD ["./rust_app"]