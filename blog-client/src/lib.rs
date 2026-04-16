mod http_client;
mod grpc_client;
mod error;

use blog_proto::*;
use crate::error::{BlogClientError};

pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClient {
    transport: Transport,
    http_client: Option<String>,
    grpc_client: Option<String>,
    token: Option<String>,
}

impl BlogClient {
    pub fn new(&self, transport: Transport) -> Self {todo!();}
    pub fn set_token(&self, token: String) -> Self {todo!()}
    pub fn get_token(&self, ) -> Option<String> {todo!()}
    pub fn register(&self, username: String, email: String, password: String) -> Result<(), BlogClientError >{todo!()}
    pub fn login(&self, username: String, password: String) -> Result<String, BlogClientError >{todo!()}

    pub fn create_post(&self, title: String, content: String) -> Result<Post, BlogClientError> {todo!()}
    pub fn get_post(&self, id: String) -> Result<Post, BlogClientError> {todo!()}
    pub fn update_post(&self, id: String, title: String, content: String) -> Result<Post, BlogClientError> {todo!()}
    pub fn delete_post(&self, id: String) -> Result<(), BlogClientError> {todo!()}
    pub fn list_posts(&self, limit: i64, offset: i64) -> Result<Post, BlogClientError> {todo!()}



}

pub struct AuthResponse {}

pub struct User {}

pub struct Post {}