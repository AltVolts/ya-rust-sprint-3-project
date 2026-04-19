use gloo_net::http::{Request, Response};
use gloo_storage::{LocalStorage, Storage};
use serde_json::json;
use crate::types::*;
use wasm_bindgen::JsValue;

const API_BASE_URL: &str = "http://localhost:3000/api";

pub fn save_token(token: &str) -> Result<(), gloo_storage::errors::StorageError> {
    LocalStorage::set("blog_token", token)
}

pub fn load_token() -> Option<String> {
    LocalStorage::get("blog_token").ok()
}

pub fn remove_token() {
    let _ = LocalStorage::delete("blog_token");
}

async fn send_request(
    method: &str,
    url: &str,
    body: Option<String>,
    token: Option<&str>,
) -> Result<Response, ApiError> {
    let full_url = format!("{}{}", API_BASE_URL, url);
    let builder = match method {
        "GET" => Request::get(&full_url),
        "POST" => Request::post(&full_url),
        "PUT" => Request::put(&full_url),
        "DELETE" => Request::delete(&full_url),
        _ => return Err(ApiError::ServerError("Unsupported method".into())),
    };

    let mut builder_with_headers = builder;
    if let Some(tok) = token {
        builder_with_headers = builder_with_headers.header("Authorization", &format!("Bearer {}", tok));
    }
    builder_with_headers = builder_with_headers.header("Content-Type", "application/json");

    let request = match method {
        "GET" | "HEAD" => {
            builder_with_headers.body(JsValue::undefined())
                .map_err(|e| ApiError::ServerError(e.to_string()))?
        }
        _ => {
            let body_str = body.unwrap_or_default();
            builder_with_headers.body(body_str)
                .map_err(|e| ApiError::ServerError(e.to_string()))?
        }
    };

    let response = request.send()
        .await
        .map_err(|e| ApiError::ServerError(e.to_string()))?;

    if response.status() >= 200 && response.status() < 300 {
        Ok(response)
    } else {
        let status = response.status();
        let text = response.text()
            .await
            .unwrap_or_else(|_| String::new());
        let err_msg = if !text.is_empty() { text } else { status.to_string() };
        match status {
            401 => Err(ApiError::Unauthorized),
            403 => Err(ApiError::Forbidden),
            404 => Err(ApiError::NotFound),
            409 => Err(ApiError::Conflict),
            _ if status >= 400 && status < 500 => Err(ApiError::BadRequest(err_msg)),
            _ => Err(ApiError::ServerError(err_msg)),
        }
    }
}

pub async fn register(username: String, email: String, password: String) -> Result<AuthResponse, ApiError> {
    let body = json!({ "username": username, "email": email, "password": password });
    let body_str = serde_json::to_string(&body).map_err(|e| ApiError::ServerError(e.to_string()))?;
    let response = send_request("POST", "/auth/register", Some(body_str), None).await?;
    let auth_resp = response.json().await.map_err(|e| ApiError::ServerError(e.to_string()))?;
    Ok(auth_resp)
}

pub async fn login(username: String, password: String) -> Result<AuthResponse, ApiError> {
    let body = json!({ "username": username, "password": password });
    let body_str = serde_json::to_string(&body).map_err(|e| ApiError::ServerError(e.to_string()))?;
    let response = send_request("POST", "/auth/login", Some(body_str), None).await?;
    let auth_resp = response.json().await.map_err(|e| ApiError::ServerError(e.to_string()))?;
    Ok(auth_resp)
}

pub async fn get_posts(limit: i64, offset: i64) -> Result<PostsListResponse, ApiError> {
    let url = format!("/posts?limit={}&offset={}", limit, offset);
    let response = send_request("GET", &url, None, None).await?;
    let posts_resp = response.json().await.map_err(|e| ApiError::ServerError(e.to_string()))?;
    Ok(posts_resp)
}

pub async fn create_post(token: &str, title: String, content: String) -> Result<Post, ApiError> {
    let body = CreatePostRequest { title, content };
    let body_str = serde_json::to_string(&body).map_err(|e| ApiError::ServerError(e.to_string()))?;
    let response = send_request("POST", "/posts", Some(body_str), Some(token)).await?;
    let post = response.json().await.map_err(|e| ApiError::ServerError(e.to_string()))?;
    Ok(post)
}

pub async fn update_post(token: &str, post_id: String, title: String, content: String) -> Result<Post, ApiError> {
    let body = UpdatePostRequest { title, content };
    let body_str = serde_json::to_string(&body).map_err(|e| ApiError::ServerError(e.to_string()))?;
    let url = format!("/posts/{}", post_id);
    let response = send_request("PUT", &url, Some(body_str), Some(token)).await?;
    let post = response.json().await.map_err(|e| ApiError::ServerError(e.to_string()))?;
    Ok(post)
}

pub async fn delete_post(token: &str, post_id: String) -> Result<(), ApiError> {
    let url = format!("/posts/{}", post_id);
    let response = send_request("DELETE", &url, None, Some(token)).await?;
    if response.status() == 204 {
        Ok(())
    } else {
        Err(ApiError::ServerError("Неожиданный ответ при удалении".into()))
    }
}