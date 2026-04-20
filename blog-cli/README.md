# Бинарное приложение-клиент для взаимодействия с сервером через cli

## Основные команды с описанием можно посмотреть через ```cargo run --  --help```

## Примеры использования:
- ```shell
  cargo run -- register --username "ivan" --email "ivan@example.com" --password "secret123" 
  ```
- ```shell
  cargo run -- login --username "ivan" --password "secret123"
  ```
  
- ```shell
  cargo run -- create --title "Мой первый пост" --content "Содержание"
  ```
- ```shell
  cargo run -- create --title "Мой первый пост" --content "Содержание" --grpc
  ```
  
- ```shell
  cargo run -- get --id 019dac5b-033f-7cb3-bf98-b91c40512f31 # подставить реальный id
  ```

- ```shell
  cargo run -- update --id 019dac5b-033f-7cb3-bf98-b91c40512f31 --title "Обновлённый заголовок" # подставить реальный id
  ```

- ```shell
  cargo run -- delete --id 019dac5b-033f-7cb3-bf98-b91c40512f31 # подставить реальный id
  ```

- ```shell
  cargo run -- list --limit 20 --offset 0
  ```

## Дополнительная информация: В проекте id имеют формат Uuid v7, нативная поддержка которого есть в Postgres 18+ версии