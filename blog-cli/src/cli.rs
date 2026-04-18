use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI для управления блогом", long_about = None)]
pub struct Cli {
    #[arg(long, global = true)]
    pub grpc: bool,

    #[arg(long, global = true)]
    pub server: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Register {
        #[arg(long)]
        username: String,
        #[arg(long)]
        email: String,
        #[arg(long)]
        password: String,
    },
    Login {
        #[arg(long)]
        username: String,
        #[arg(long)]
        password: String,
    },
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        content: String,
    },
    Get {
        #[arg(long)]
        id: String,
    },
    Update {
        #[arg(long)]
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        content: Option<String>,
    },
    Delete {
        #[arg(long)]
        id: String,
    },
    List {
        #[arg(long, default_value_t = 10)]
        limit: i64,
        #[arg(long, default_value_t = 0)]
        offset: i64,
    },
}

impl Cli {
    pub fn get_args() -> Self {
        Self::parse()
    }
}
