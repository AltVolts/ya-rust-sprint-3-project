use crate::TransportClient;
use crate::dto::{
    AuthResponse, CreatePost, LoginUser, PaginatedPosts, Post, RegisterResponse, RegisterUser,
    UpdatePost,
};
use crate::error::BlogClientError;
use async_trait::async_trait;
use reqwest::{Response, StatusCode};

pub(crate) struct HttpClient {
    client: reqwest::Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .build()
            .expect("HTTP client creation failed");
        HttpClient {
            client,
            base_url: base_url.into(),
        }
    }
}
#[async_trait]
impl TransportClient for HttpClient {
    async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, BlogClientError> {
        let url = format!("{}/auth/register", self.base_url.trim_end_matches('/'));
        let req_body = RegisterUser {
            username,
            email,
            password,
        };
        let response = self.client.post(url).json(&req_body).send().await?;
        let response = response.error_for_status_blog().await?;
        let body: RegisterResponse = response.json().await?;

        Ok(body)
    }

    async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let url = format!("{}/auth/login", self.base_url.trim_end_matches('/'));
        let req_body = LoginUser { username, password };
        let response = self.client.post(url).json(&req_body).send().await?;
        let response = response.error_for_status_blog().await?;
        let body: AuthResponse = response.json().await?;
        Ok(body)
    }

    async fn create_post(
        &mut self,
        token: Option<String>,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let url = format!("{}/posts", self.base_url.trim_end_matches('/'));
        let req_body = CreatePost { title, content };
        let mut req = self.client.post(url).json(&req_body);
        if let Some(tok) = token {
            req = req.bearer_auth(tok);
        }
        let response = req.send().await?;
        let response = response.error_for_status_blog().await?;
        let body: Post = response.json().await?;
        Ok(body)
    }

    async fn get_post(&mut self, id: String) -> Result<Post, BlogClientError> {
        let url = format!("{}/posts/{}", self.base_url.trim_end_matches('/'), id);
        let response = self.client.get(url).send().await?;
        let response = response.error_for_status_blog().await?;
        let body: Post = response.json().await?;
        Ok(body)
    }

    async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedPosts, BlogClientError> {
        let url = format!("{}/posts", self.base_url.trim_end_matches('/'));
        let response = self
            .client
            .get(url)
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;
        let response = response.error_for_status_blog().await?;
        let body: PaginatedPosts = response.json().await?;
        Ok(body)
    }

    async fn update_post(
        &mut self,
        token: Option<String>,
        id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Post, BlogClientError> {
        let url = format!("{}/posts/{}", self.base_url.trim_end_matches('/'), id);
        let req_body = UpdatePost { title, content };
        let mut req = self.client.put(url).json(&req_body);
        if let Some(tok) = token {
            req = req.bearer_auth(tok);
        }
        let response = req.send().await?;
        let response = response.error_for_status_blog().await?;
        let body: Post = response.json().await?;
        Ok(body)
    }

    async fn delete_post(
        &mut self,
        token: Option<String>,
        id: String,
    ) -> Result<(), BlogClientError> {
        let url = format!("{}/posts/{}", self.base_url.trim_end_matches('/'), id);
        let mut req = self.client.delete(url);
        if let Some(tok) = token {
            req = req.bearer_auth(tok);
        }
        let response = req.send().await?;
        response.error_for_status_blog().await?;
        Ok(())
    }
}

trait ResponseExt {
    async fn error_for_status_blog(self) -> Result<Response, BlogClientError>;
}

impl ResponseExt for Response {
    async fn error_for_status_blog(self) -> Result<Response, BlogClientError> {
        let status = self.status();
        if status == StatusCode::UNAUTHORIZED {
            Err(BlogClientError::Unauthorized)
        } else if status == StatusCode::NOT_FOUND {
            Err(BlogClientError::NotFound)
        } else if !status.is_success() {
            let text = self.text().await.map_err(BlogClientError::HttpError)?;
            Err(BlogClientError::InvalidRequest(text))
        } else {
            Ok(self)
        }
    }
}
