# Лабораторная работа 11 Вариант 9
## Выполнил студент гр 221131 Кривохин Артём Дмитриевич



Этот проект содержит два веб-приложения, упакованных в Docker-контейнеры:
1. **Python Flask приложение** – простое веб-приложение на Python с эндпоинтами здоровья и информации о системе.
2. **Rust приложение** – веб-сервер на Rust с использованием фреймворка warp, предоставляющий аналогичные эндпоинты плюс калькулятор.

## Структура проекта

```
mid/
├── python_app/               # Папка с Python-приложением и Docker-конфигурацией
│   ├── app.py               # Основное Flask-приложение
│   ├── test_app.py          # Тесты для приложения
│   ├── Dockerfile           # Docker-конфигурация
│   ├── requirements.txt     # Зависимости Python
│   ├── .dockerignore        # Игнорируемые файлы для Docker
│   └── docker-compose.yml   # Docker Compose с ограничениями ресурсов
├── Rust_app/                # Папка с Rust-приложением и Docker-конфигурацией
│   ├── Cargo.toml           # Манифест Rust-проекта
│   ├── src/main.rs          # Исходный код веб-сервера на Rust
│   ├── Dockerfile           # Многоступенчатый Docker-образ
│   ├── .dockerignore        # Игнорируемые файлы для Docker
│   └── docker-compose.yml   # Docker Compose с ограничениями ресурсов
└── docker-compose.yml       # Общий Docker Compose для обоих приложений
hard/
└── go_project/              # Папка с Go-приложением и Docker-конфигурацией (задание повышенной сложности)
    ├── main.go              # Исходный код Go-приложения
    ├── main_test.go         # Unit-тесты
    ├── go.mod               # Модуль Go
    ├── go.sum               # Зависимости
    ├── Dockerfile           # Многостадийный Dockerfile (builder + scratch)
    ├── .dockerignore        # Игнорируемые файлы для Docker
    ├── Makefile             # Управление сборкой
    ├── build.sh             # Скрипт сборки для Linux/macOS
    └── build.bat            # Скрипт сборки для Windows
```

## Требования

- Docker Desktop (или Docker Engine) версии 20.10+
- Для локальной разработки:
  - Python 3.11 (для Python-приложения)
  - Rust и Cargo (для Rust-приложения)
  - Go 1.23+ (для Go-приложения, опционально)

--- 
Задание 1 и 3(средняя) Написать Dockerfile для Python-приложения, Написать Dockerfile для Rust-приложения
# Python Flask приложение

## Быстрый старт

### 1. Сборка Docker-образа

```bash
cd mid/python_app
docker build -t python-flask-app .
```

### 2. Запуск контейнера

```bash
docker run -d -p 5000:5000 --name flask-app python-flask-app
```

### 3. Проверка работы приложения

Откройте браузер и перейдите по адресу:
- http://localhost:5000/ - главная страница
- http://localhost:5000/health - проверка здоровья
- http://localhost:5000/info - информация о системе

Или используйте curl:

```bash
curl http://localhost:5000/
```

## API эндпоинты

- `GET /` – Главная страница с приветствием и информацией о хосте
- `GET /health` – Проверка здоровья приложения (возвращает `{"status": "healthy"}`)
- `GET /info` – Информация о системе (версия Python, платформа, рабочая директория)

## Локальная разработка (без Docker)

### Установка зависимостей

```bash
cd mid/python_app
pip install -r requirements.txt
```

### Запуск приложения

```bash
python app.py
```

Приложение будет доступно по адресу http://localhost:5000

### Запуск тестов

```bash
python -m unittest test_app.py
```

---

# Rust приложение

## Быстрый старт

### 1. Сборка Docker-образа

```bash
cd mid/Rust_app
docker build -t rust-app .
```

### 2. Запуск контейнера

```bash
docker run -d -p 8080:8080 --name rust-app-container rust-app
```

### 3. Проверка работы приложения

Откройте браузер и перейдите по адресу:
- http://localhost:8080/ – главная страница
- http://localhost:8080/health – проверка здоровья
- http://localhost:8080/info – информация о системе
- http://localhost:8080/calc/{x}/{op}/{y} – калькулятор (операции: add, subtract, multiply, divide)

Пример:
```bash
curl http://localhost:8080/calc/5/add/3
```
Ответ:
```json
{"result":8.0,"operation":"add","x":5.0,"y":3.0}
```

## API эндпоинты

- `GET /` – Главная страница с приветствием, версией, именем хоста и окружением
- `GET /health` – Проверка здоровья (`{"status": "healthy"}`)
- `GET /info` – Информация о системе (версия Rust, платформа, рабочая директория, пользователь)
- `GET /calc/{x}/{op}/{y}` – Калькулятор с поддержкой операций `add`, `subtract`, `multiply`, `divide`

## Локальная разработка (без Docker)

### Установка зависимостей

Убедитесь, что установлен Rust и Cargo. Затем:

```bash
cd mid/Rust_app
cargo build --release
```

### Запуск приложения

```bash
cargo run --release
```

Приложение будет доступно по адресу http://localhost:8080 (порт можно изменить через переменную окружения `PORT`).

### Запуск тестов

```bash
cargo test
```

Модульные тесты проверяют логику калькулятора. Интеграционные тесты находятся в папке `tests/` и требуют доработки.

---

## Docker команды (общие)

### Сборка образа
```bash
cd mid/<папка_приложения>
docker build -t <имя_образа> .
```

### Запуск контейнера
```bash
docker run -d -p <локальный_порт>:<порт_контейнера> --name <имя_контейнера> <имя_образа>
```

### Просмотр логов
```bash
docker logs <имя_контейнера>
```

### Остановка контейнера
```bash
docker stop <имя_контейнера>
```

### Удаление контейнера
```bash
docker rm <имя_контейнера>
```

### Удаление образа
```bash
docker rmi <имя_образа>
```

## Переменные окружения

Оба приложения поддерживают следующие переменные окружения:

- `PORT` – порт для запуска приложения (по умолчанию: Python – 5000, Rust – 8080)
- `HOST` – хост для привязки (по умолчанию 0.0.0.0)
- `ENV` – окружение (development/production, по умолчанию development)

Пример запуска Rust-приложения с переменными окружения:

```bash
docker run -d -p 9090:9090 -e PORT=9090 -e ENV=production --name rust-app rust-app
```

## Многоступенчатая сборка Docker

Оба Dockerfile используют многоступенчатую сборку:
1. **Builder stage** – установка системных зависимостей и компиляция приложения
2. **Final stage** – создание минимального образа с копированием только необходимых файлов

Это позволяет уменьшить размер итогового образа.

## Задание 9(средняя). Ограничить ресурсы (CPU, память) для контейнеров

Для управления ресурсами контейнеров (CPU и память) были созданы файлы `docker-compose.yml` в каждой папке приложения и общий файл в папке `mid/`.

### Ограничения ресурсов в docker-compose

В файлах `docker-compose.yml` используются следующие настройки:

#### Python приложение (`mid/python_app/docker-compose.yml`):
```yaml
deploy:
  resources:
    limits:
      cpus: '0.5'          # Ограничение до 0.5 CPU ядра
      memory: 256M         # Ограничение памяти до 256 МБ
    reservations:
      cpus: '0.1'          # Резервирование минимум 0.1 CPU
      memory: 128M         # Резервирование минимум 128 МБ памяти
```

#### Rust приложение (`mid/Rust_app/docker-compose.yml`):
```yaml
deploy:
  resources:
    limits:
      cpus: '1.0'          # Ограничение до 1 CPU ядра
      memory: 512M         # Ограничение памяти до 512 МБ
    reservations:
      cpus: '0.2'          # Резервирование минимум 0.2 CPU
      memory: 256M         # Резервирование минимум 256 МБ памяти
```

### Запуск с ограничениями ресурсов

1. **Запуск отдельного приложения** (например, Python):
   ```bash
   cd mid/python_app
   docker-compose up -d --build
   ```

2. **Запуск обоих приложений одновременно**:
   ```bash
   cd mid
   docker-compose up -d --build
   ```

3. **Проверка ограничений ресурсов**:
   ```bash
   docker stats --no-stream python_app_container
   docker stats --no-stream rust_app_container
   ```

### Ограничения через docker run

Также можно задавать ограничения напрямую через команду `docker run`:

```bash
# Ограничение памяти и CPU
docker run -d -p 5000:5000 --memory=256m --cpus=0.5 --name flask-app python-flask-app

# Ограничение памяти без ограничения CPU
docker run -d -p 8080:8080 --memory=512m --name rust-app rust-app
```

### Проверка работы ограничений

После запуска контейнеров можно убедиться, что ограничения применяются:

```bash
$ docker stats --no-stream python_app_container
CONTAINER ID   NAME                   CPU %     MEM USAGE / LIMIT   MEM %     NET I/O        BLOCK I/O     PIDS
b6aeb863590d   python_app_container   0.02%     25.84MiB / 256MiB   10.09%    1.7kB / 126B   22.5MB / 0B   1
```

Видно, что лимит памяти установлен на 256MiB, а использование составляет 25.84MiB.

### Дополнительные настройки

- **Healthcheck**: добавлены проверки здоровья для мониторинга состояния приложений.
- **Сети**: создана изолированная сеть `app_network` для каждого приложения.
- **Перезапуск**: политика `restart: unless-stopped` обеспечивает автоматический перезапуск при сбоях.

## Задание 1 (повышенная). Собрать Go-приложение с поддержкой статической компиляции и запустить в scratch-образе

В папке `hard/go_project` реализовано Go-приложение с поддержкой статической компиляции и запуском в scratch-образе.

### Структура проекта

```
hard/go_project/
├── main.go                 # Исходный код Go-приложения
├── main_test.go            # Unit-тесты
├── go.mod                  # Модуль Go
├── go.sum                  # Зависимости
├── Dockerfile              # Многостадийный Dockerfile (builder + scratch)
├── .dockerignore           # Игнорируемые файлы для Docker
├── Makefile                # Управление сборкой (build, test, clean)
├── build.sh                # Скрипт сборки для Linux/macOS
└── build.bat               # Скрипт сборки для Windows
```

### Функциональность приложения

- **HTTP-сервер** на порту 8000 (настраивается через переменную окружения `PORT`)
- **Эндпоинты**:
  - `GET /` – приветствие с параметром `name` (по умолчанию "World")
  - `GET /health` – статус здоровья приложения (JSON)
  - `GET /version` – версия и тип сборки (JSON)
- **Статическая компиляция** – бинарник не зависит от системных библиотек

### Сборка статического бинарника

#### Linux/macOS
```bash
cd hard/go_project
./build.sh
```
или
```bash
make build-static
```

#### Windows
```cmd
cd hard\go_project
build.bat
```
или
```powershell
cd hard/go_project
make build-static
```

### Docker-образ на основе scratch

Dockerfile использует двухстадийную сборку:
1. **Builder stage** – компиляция статического бинарника с `CGO_ENABLED=0`
2. **Scratch stage** – копирование бинарника и CA-сертификатов в пустой образ

#### Сборка образа
```bash
cd hard/go_project
docker build -t go-app:latest .
```

#### Запуск контейнера
```bash
docker run -d -p 8000:8000 --name go-app-test go-app:latest
```

### Тестирование

#### Запуск unit-тестов
```bash
cd hard/go_project
go test -v
```

#### Проверка работы контейнера
- **Linux/macOS**:
  ```bash
  curl http://localhost:8000/
  curl http://localhost:8000/health
  curl http://localhost:8000/version
  ```
- **Windows (PowerShell)**:
  ```powershell
  Invoke-WebRequest -Uri "http://localhost:8000/" -UseBasicParsing
  Invoke-WebRequest -Uri "http://localhost:8000/health" -UseBasicParsing
  Invoke-WebRequest -Uri "http://localhost:8000/version" -UseBasicParsing
  ```

### Результат
- Статический бинарник `go-app` (≈5 МБ)
- Docker-образ `go-app:latest` на основе scratch (≈5.5 МБ)
- Полное покрытие тестами
- Работает на любом Linux-хосте без зависимостей

## Задание 3 (повышенная). Настроить CI/CD, который собирает и пушит образы для всех трёх языков

Реализован CI/CD pipeline с использованием GitHub Actions для автоматической сборки, тестирования и публикации Docker-образов всех трёх приложений (Go, Rust, Python) в GitHub Container Registry.

### Конфигурация workflow
Файл `.github/workflows/docker-build-push.yml` содержит конфигурацию, которая:

1. **Триггеры**:
   - Пуш в ветки `main`/`master`
   - Создание тегов (v*)
   - Ручной запуск (workflow_dispatch)
   - Pull request в main/master

2. **Параллельная сборка** с использованием матрицы:
   - Go-приложение (`hard/go_project`)
   - Python Flask-приложение (`mid/python_app`)
   - Rust-приложение (`mid/Rust_app`)

3. **Этапы выполнения** для каждого приложения:
   - Checkout кода
   - Настройка Docker Buildx
   - Логин в GitHub Container Registry (только для не-PR событий)
   - Извлечение метаданных для тегов и лейблов
   - Сборка и push Docker-образа с кэшированием
   - Запуск unit-тестов

4. **Тегирование образов**:
   - `latest` для ветки main/master
   - Тег по SHA коммита
   - Семантическое версионирование (v1.0.0, v1.0, v1)
   - Теги для pull request

### Используемые технологии
- **GitHub Actions** – оркестрация CI/CD
- **Docker Buildx** – многоплатформенная сборка
- **GitHub Container Registry (ghcr.io)** – хранение образов
- **Кэширование GHA** – ускорение повторных сборок

### Имена образов
После успешного выполнения workflow образы публикуются по адресам:
- `ghcr.io/<ваш-username>/go-app:latest`
- `ghcr.io/<ваш-username>/python-app:latest`
- `ghcr.io/<ваш-username>/rust-app:latest`

### Запуск тестов
В процессе сборки автоматически выполняются тесты для каждого языка:
- **Go**: `go test -v ./...`
- **Python**: `python -m unittest test_app.py -v`
- **Rust**: `cargo test --release --verbose`

### Преимущества реализации
- **Полная автоматизация** – образы собираются и публикуются без ручного вмешательства
- **Многоплатформенная поддержка** – возможность сборки для разных архитектур
- **Кэширование зависимостей** – ускорение сборки Rust и Python приложений
- **Интеграция с GitHub экосистемой** – использование встроенного GITHUB_TOKEN для аутентификации
- **Масштабируемость** – легко добавить новые приложения или изменить конфигурацию

### Пример успешного выполнения
```yaml
name: Build and Push Docker Images
on: [push, pull_request, workflow_dispatch]
jobs:
  build-and-push:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        project: [go-app, python-app, rust-app]
    steps:
      - name: Build and push
        uses: docker/build-push-action@v5
```

## Тестирование

- **Python**: unit-тесты проверяют корректность ответов эндпоинтов, статус-коды HTTP и структуру JSON-ответов.
- **Rust**: модульные тесты проверяют логику калькулятора; интеграционные тесты (в папке `tests/`) требуют доработки.

Для запуска тестов внутри контейнера (Python):

```bash
docker run --rm python-flask-app python -m unittest test_app.py
```

Для Rust тестов они выполняются на этапе сборки Docker-образа.

## Примечания

- Dockerfile и .dockerignore расположены внутри папок `python_app` и `Rust_app` для изоляции конфигурации Docker от остальных частей проекта.
- При сборке образа убедитесь, что вы находитесь в директории соответствующего приложения, так как Dockerfile использует относительные пути.
- Rust-приложение использует фреймворк warp и tokio для асинхронного веб-сервера.

## Лицензия

MIT