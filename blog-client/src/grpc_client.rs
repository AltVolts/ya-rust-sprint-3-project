use crate::dto::{AuthResponse, PaginatedPosts, Post, RegisterResponse};
use crate::error::BlogClientError;
use blog_proto::blog_service_client::BlogServiceClient;
use blog_proto::{
    CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest, LoginRequest,
    RegisterRequest, UpdatePostRequest,
};
use tonic::{Request, async_trait};

pub struct GrpcClient {
    client: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub async fn new(addr: String) -> Result<Self, tonic::transport::Error> {
        let client = BlogServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
}

#[async_trait]
impl crate::TransportClient for GrpcClient {
    async fn register(
        &mut self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegisterResponse, BlogClientError> {
        let request = Request::new(RegisterRequest {
            username,
            password,
            email,
        });

        let response = self.client.register(request).await?;
        let grpc_body = response.into_inner();
        let grpc_user = grpc_body.user.ok_or(BlogClientError::InvalidResponse(
            "User not found".to_string(),
        ))?;

        Ok(RegisterResponse {
            user: grpc_user.into(),
            token: grpc_body.token,
        })
    }

    async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<AuthResponse, BlogClientError> {
        let request = Request::new(LoginRequest { username, password });
        let response = self.client.login(request).await?;
        let access_token = response.into_inner().access_token;

        Ok(AuthResponse { access_token })
    }

    async fn create_post(
        &mut self,
        token: Option<String>,
        title: String,
        content: String,
    ) -> Result<Post, BlogClientError> {
        let mut request = Request::new(CreatePostRequest { title, content });

        add_auth_meta(&mut request, token)?;

        let response = self.client.create_post(request).await?;
        let grpc_post = response.into_inner();

        Ok(Post::try_from(grpc_post)?)
    }

    async fn get_post(&mut self, id: String) -> Result<Post, BlogClientError> {
        let request = Request::new(GetPostRequest { id });

        let response = self.client.get_post(request).await?;
        let grpc_post = response.into_inner();
        Ok(Post::try_from(grpc_post)?)
    }

    async fn list_posts(
        &mut self,
        limit: i64,
        offset: i64,
    ) -> Result<PaginatedPosts, BlogClientError> {
        let request = Request::new(ListPostsRequest { limit, offset });
        let response = self.client.list_posts(request).await?;
        let grpc_posts = response.into_inner();
        let posts: Vec<Post> = grpc_posts
            .posts
            .into_iter()
            .map(Post::try_from)
            .collect::<Result<Vec<Post>, BlogClientError>>()?;
        let posts_result = PaginatedPosts {
            posts,
            total: grpc_posts.total,
        };
        Ok(posts_result)
    }

    async fn update_post(
        &mut self,
        token: Option<String>,
        id: String,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<Post, BlogClientError> {
        let mut request = Request::new(UpdatePostRequest { id, title, content });

        add_auth_meta(&mut request, token)?;

        let response = self.client.update_post(request).await?;
        let grpc_post = response.into_inner();
        Ok(Post::try_from(grpc_post)?)
    }

    async fn delete_post(
        &mut self,
        token: Option<String>,
        id: String,
    ) -> Result<(), BlogClientError> {
        let mut request = Request::new(DeletePostRequest { id });
        add_auth_meta(&mut request, token)?;
        self.client.delete_post(request).await?;
        Ok(())
    }
}

fn add_auth_meta<T>(
    request: &mut Request<T>,
    token: Option<String>,
) -> Result<(), BlogClientError> {
    if let Some(tok) = token {
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", tok)
                .parse()
                .map_err(|_| BlogClientError::InvalidRequest("Invalid token".into()))?,
        );
    }
    Ok(())
}
