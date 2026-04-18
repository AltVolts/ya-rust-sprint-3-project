mod dto;
mod error;
mod grpc_client;
mod http_client;

use crate::dto::{AuthResponse, PaginatedPosts, Post, RegisterResponse};
use crate::error::BlogClientError;
use async_trait::async_trait;

pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClient {
    transport: Transport,
    client: Box<dyn TransportClient + Send + Sync>,
    token: Option<String>,
}

impl BlogClient {
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let client: Box<dyn TransportClient + Send + Sync> = match &transport {
            Transport::Http(addr) => Box::new(http_client::HttpClient::new(addr)),
            Transport::Grpc(addr) => {
                let grpc_client = grpc_client::GrpcClient::new(addr.clone())
                    .await
                    .map_err(BlogClientError::TransportError)?;
                Box::new(grpc_client)
            }
        };
        Ok(Self {
            transport,
            client,
            token: None,
        })
    }
    fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
    fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, BlogClientError> {
        let response = self.client.register(username, email, password).await?;
        Ok(response)
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let response = self.client.login(username, password).await?;
        self.set_token(response.token.clone());
        Ok(response)
    }

    pub async fn create_post(
        &mut self,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let token = self.get_token();
        let post = self.client.create_post(token, title, content).await?;
        Ok(post)
    }

    pub async fn get_post(&mut self, post_id: String) -> Result<Post, BlogClientError> {
        let post = self.client.get_post(post_id).await?;
        Ok(post)
    }

    pub async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedPosts, BlogClientError> {
        let post_list = self.client.list_posts(limit, offset).await?;
        Ok(post_list)
    }

    pub async fn update_post(
        &mut self,
        post_id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Post, BlogClientError> {
        let token = self.get_token();
        let updated_post = self
            .client
            .update_post(token, post_id, title, content)
            .await?;
        Ok(updated_post)
    }

    pub async fn delete_post(&mut self, post_id: String) -> Result<(), BlogClientError> {
        self.client.delete_post(self.get_token(), post_id).await
    }
}

#[async_trait]
pub trait TransportClient: Send + Sync {
    async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, BlogClientError>;
    async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError>;
    async fn create_post(
        &mut self,
        token: Option<String>,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError>;
    async fn get_post(&mut self, id: String) -> Result<Post, BlogClientError>;
    async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedPosts, BlogClientError>;
    async fn update_post(
        &mut self,
        token: Option<String>,
        id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Post, BlogClientError>;
    async fn delete_post(
        &mut self,
        token: Option<String>,
        id: String,
    ) -> Result<(), BlogClientError>;
}
