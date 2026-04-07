# Используем официальный образ Python как базовый
FROM python:3.11-slim AS builder

# Устанавливаем системные зависимости
RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    && rm -rf /var/lib/apt/lists/*

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем файл зависимостей (находится в той же директории, что и Dockerfile)
COPY requirements.txt .

# Устанавливаем зависимости Python
RUN pip install --no-cache-dir --user -r requirements.txt

# Второй этап: создаем финальный образ
FROM python:3.11-slim

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем установленные зависимости из builder
COPY --from=builder /root/.local /root/.local

# Копируем исходный код приложения (все файлы из текущей директории)
COPY . .

# Добавляем директорию с зависимостями в PATH
ENV PATH=/root/.local/bin:$PATH

# Открываем порт, который будет использовать приложение
EXPOSE 5000

# Устанавливаем переменные окружения
ENV FLASK_APP=app.py
ENV FLASK_ENV=production
ENV PORT=5000
ENV HOST=0.0.0.0

# Запускаем приложение
CMD ["python", "app.py"]