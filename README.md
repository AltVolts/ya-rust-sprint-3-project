# Blog System – Full‑stack проект на Rust

Данный проект представляет собой полноценную систему блога, реализованную на Rust с использованием современных технологий: HTTP (actix-web) и gRPC (tonic) серверы, JWT‑аутентификация, Postgres, клиентская библиотека, CLI‑инструмент и WASM‑фронтенд. Проект разбит на четыре крейта в едином Cargo workspace.

Proto файл со схемой описания взаимодействия клиентской и серверной частей по grpc, генерация кода rust кода для обеих частей системы вынесена в отдельный крейт blog-proto, импортируемый blog-server и blog-client.

## Архитектура
```
blog-project/
├── .env
├── .blog_token (optional)
├── docker-compose.yml
├── Cargo.toml
├── README.md
├── blog-server/
├── blog-client/
├── blog-cli/
├── blog-wasm/
└── blog-proto/ 
```
### Компоненты

- **blog-server** – основной сервер:
    - HTTP API на порту `3000` (actix-web)
    - gRPC API на порту `50051` (tonic)
    - JWT‑аутентификация (jsonwebtoken + Argon2)
    - PostgreSQL (sqlx) с миграциями
    - CORS для работы с фронтендом
- **blog-client** – библиотека для взаимодействия с сервером через HTTP или gRPC.
- **blog-cli** – CLI-клиент, использующий `blog-client`.
- **blog-wasm** – фронтенд на WebAssembly (yew), взаимодействует с сервером по HTTP.
- **blog-proto** – фронтенд на WebAssembly (yew), взаимодействует с сервером по HTTP.

## Требования
- Rust (последняя стабильная версия)
- Postgres (версия 18+)
- trunk (для сборки WASM и для запуска отладочного сервера)
- **trunk** (для сборки WASM и запуска dev-сервера) – установка: `cargo install trunk`
- WASM target: `rustup target add wasm32-unknown-unknown`
- **Важно:** при разработке фронтенда бэкенд должен быть запущен на `http://localhost:3000` (стандартный порт для `blog-server`).
- protoc (для работы с protobuf)
- выполнить rustup target add wasm32-unknown-unknown(для компиляции wasm)
- docker и docker compose для запуска БД из docker-compose.yml файла

## Настройка окружения
Необходимые переменные окружения описаны в .env.example, по образу которого необходимо создать .env файл в корне проекта.
Данные переменные используется как в коде Rust на этапе инициализации программ, так и в docker compose файле для инициализации БД.
Генерация jwt токенов в коде сервера основывается на значение ключа из .env.

## Пример команд (из корня проекта):
### 1) БД: 
```bash 
  docker compose up -d
```
### 2) Сервер:
```bash 
  cargo server
```
### 3) Cli клиент:
```bash 
  cargo blog-cli help # показать описание и возможные команды утилиты
```

### 4) Frontend:

#### Для запуска отладочного сервера с автоматической пересборкой WASM:

```bash
  cd blog-wasm && trunk serve --open   # для bash/zsh
```

```shell
  cd blog-wasm; trunk serve --open     # для PowerShell
```




## Порядок запуска

1. Клонируем проект ```git clone git@github.com:AltVolts/ya-rust-sprint-3-project.git```
2. Создаем .env файл на основе .env.example ```cp .env.example .env```
3. Редактируем ```.env``` файл
4. Запускаем бд ```docker compose up -d```
5. Билдим проект ```cargo build --workspace```
6. Запускаем сервер ```cargo server```
7. Запускаем trunk сервер с приложением ```cd blog-wasm; trunk serve --open ```. После запуска frontend приложение будет доступно по адресу http://localhost:8080
8. Используем cli программу как ```cargo blog-cli <command> [<options>]```
