mod cli;
mod token;

use crate::cli::{Cli, Commands};
use crate::token::{load_token, save_token};
use blog_client::{BlogClient, Transport};
use std::process::exit;

#[tokio::main]
async fn main() {
    let cli = Cli::get_args();

    let default_addr = if cli.grpc {
        "http://localhost:50051/api".to_string()
    } else {
        "http://localhost:3000/api".to_string()
    };
    let server_addr = cli.server.unwrap_or(default_addr);

    let transport = if cli.grpc {
        Transport::Grpc(server_addr)
    } else {
        Transport::Http(server_addr)
    };

    let mut client = match BlogClient::new(transport).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Ошибка создания клиента: {}", e);
            exit(1);
        }
    };

    if let Some(token) = load_token() {
        client.set_token(token);
    }

    match cli.command {
        Commands::Register {
            username,
            email,
            password,
        } => match client.register(username, email, password).await {
            Ok(response) => {
                if let Err(e) = save_token(&response.token) {
                    eprintln!("Не удалось сохранить токен: {}", e);
                    exit(1);
                }

                println!("Зарегистрирован пользователь: {}", response.user);
            }
            Err(e) => {
                eprintln!("Ошибка регистрации: {}", e);
                exit(1);
            }
        },
        Commands::Login { username, password } => match client.login(username, password).await {
            Ok(response) => {
                if let Err(e) = save_token(&response.token) {
                    eprintln!("Не удалось сохранить токен: {}", e);
                    exit(1);
                }
                println!(
                    "Пользователь {} с ID <{}> залогинился",
                    response.user, response.user.id
                );
            }
            Err(e) => {
                eprintln!("Ошибка входа: {}", e);
                exit(1);
            }
        },
        Commands::Create { title, content } => match client.create_post(title, content).await {
            Ok(post) => {
                println!("Пост создан: \n{}", post);
            }
            Err(e) => {
                eprintln!("Ошибка создания поста: {}", e);
                exit(1);
            }
        },
        Commands::Get { id } => match client.get_post(id.to_string()).await {
            Ok(post) => {
                println!("Пост получен: \n{}", post);
            }
            Err(e) => {
                eprintln!("Ошибка получения поста: {}", e);
                exit(1);
            }
        },
        Commands::Update { id, title, content } => {
            match client.update_post(id.to_string(), title, content).await {
                Ok(post) => {
                    println!("Пост обновлён: \n{}", post);
                }
                Err(e) => {
                    eprintln!("Ошибка обновления поста: {}", e);
                    exit(1);
                }
            }
        }
        Commands::Delete { id } => match client.delete_post(id.to_string()).await {
            Ok(()) => {
                println!("Пост c ID {} удалён.", id);
            }
            Err(e) => {
                eprintln!("Ошибка удаления поста: {}", e);
                exit(1);
            }
        },
        Commands::List { limit, offset } => match client.list_posts(limit, offset).await {
            Ok(paginated) => {
                println!("Список постов (всего: {}):", paginated.total);
                for post in paginated.posts {
                    println!("{}", post);
                    println!("---\n");
                }
            }
            Err(e) => {
                eprintln!("Ошибка получения списка: {}", e);
                exit(1);
            }
        },
    }
}
