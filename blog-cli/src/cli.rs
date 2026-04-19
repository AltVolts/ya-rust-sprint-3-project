use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "blog-cli", about = "CLI для управления блогом", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Иcпользовать grpc протокол")]
    pub grpc: bool,

    #[arg(short, long, global = true, help = "Url aдрес сервера")]
    pub server: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Регистрация нового пользователя
    Register {
        #[arg(long, help = "Имя пользователя")]
        username: String,
        #[arg(long, help = "Электронная почта")]
        email: String,
        #[arg(long, help = "Пароль")]
        password: String,
    },
    /// Вход в систему (получение токена)
    Login {
        #[arg(long, help = "Имя пользователя")]
        username: String,
        #[arg(long, help = "Пароль")]
        password: String,
    },
    /// Создание нового поста
    Create {
        #[arg(long, help = "Заголовок поста")]
        title: String,
        #[arg(long, help = "Содержимое поста")]
        content: String,
    },
    /// Получение поста по ID
    Get {
        #[arg(long, help = "Идентификатор поста")]
        id: String,
    },
    /// Обновление существующего поста
    Update {
        #[arg(long, help = "Идентификатор поста")]
        id: String,
        #[arg(long, help = "Новый заголовок")]
        title: Option<String>,
        #[arg(long, help = "Новое содержимое")]
        content: Option<String>,
    },
    /// Удаление поста по ID
    Delete {
        #[arg(long, help = "Идентификатор поста")]
        id: String,
    },
    /// Получение списка постов с пагинацией
    List {
        #[arg(long, default_value_t = 10, help = "Максимальное количество постов")]
        limit: i64,
        #[arg(long, default_value_t = 0, help = "Смещение для пагинации")]
        offset: i64,
    },
}

impl Cli {
    pub fn get_args() -> Self {
        Self::parse()
    }
}
