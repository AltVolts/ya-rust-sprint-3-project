use crate::api;
use crate::components::{LoginForm, PostForm, PostList, RegisterForm};
use crate::types::{Post, User};
use yew::prelude::*;

pub enum AppMsg {
    LoginSuccess(User),
    RegisterSuccess(User),
    Logout,
    LoadPosts,
    PostsLoaded(Vec<Post>),
    CreatePost(String, String),
    UpdatePost(String, String, String),
    DeletePost(String),
    EditPost(Post),
    ClearEditing,
    Error(String),
}

pub struct BlogApp {
    current_user: Option<User>,
    token: Option<String>,
    posts: Vec<Post>,
    editing_post: Option<Post>,
    error: Option<String>,
    success: Option<String>,
}

impl Component for BlogApp {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        if let Some(token) = api::load_token() {
            // Попробуем загрузить пользователя? API не имеет /me, поэтому просто будем считать,
            // что если токен есть, то пользователь аутентифицирован. Но для отображения имени
            // пользователя нужно сохранить user при логине. Поэтому токен без user - неполный.
            // Для простоты при инициализации из localStorage токен загружаем, но пользователя нет.
            // Предлагаю хранить user в localStorage тоже, но в ТЗ только токен.
            // Решение: при загрузке страницы, если есть токен, но нет user, попробуем получить user
            // через отдельный эндпоинт /auth/me (но его нет). Поэтому проще при инициализации
            // не пытаться восстановить сессию, а требовать повторный логин. Или добавить сохранение user в localStorage.
            // Для упрощения – будем сохранять user вместе с токеном.
            // Изменим api::save_token, чтобы сохранять и user. Но сейчас оставим так:
            // Показываем, что не аутентифицирован.
            Self {
                current_user: None,
                token: Some(token),
                posts: vec![],
                editing_post: None,
                error: None,
                success: None,
            }
        } else {
            Self {
                current_user: None,
                token: None,
                posts: vec![],
                editing_post: None,
                error: None,
                success: None,
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::LoginSuccess(user) => {
                self.current_user = Some(user.clone());
                self.token = api::load_token(); // токен уже сохранён в login_form
                self.error = None;
                ctx.link().send_message(AppMsg::LoadPosts);
                true
            }
            AppMsg::RegisterSuccess(user) => {
                self.current_user = Some(user);
                self.token = api::load_token();
                self.error = None;
                ctx.link().send_message(AppMsg::LoadPosts);
                true
            }
            AppMsg::Logout => {
                api::remove_token();
                self.current_user = None;
                self.token = None;
                self.posts = vec![];
                self.editing_post = None;
                self.error = None;
                self.success = Some("Вы вышли из системы".into());
                true
            }
            AppMsg::LoadPosts => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match api::get_posts(100, 0).await {
                        Ok(resp) => link.send_message(AppMsg::PostsLoaded(resp.posts)),
                        Err(e) => link.send_message(AppMsg::Error(e.to_string())),
                    }
                });
                false
            }
            AppMsg::PostsLoaded(posts) => {
                self.posts = posts;
                self.error = None;
                true
            }
            AppMsg::CreatePost(title, content) => {
                if let Some(token) = &self.token {
                    let link = ctx.link().clone();
                    let token = token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::create_post(&token, title, content).await {
                            Ok(_) => link.send_message(AppMsg::LoadPosts),
                            Err(e) => link.send_message(AppMsg::Error(e.to_string())),
                        }
                    });
                } else {
                    ctx.link()
                        .send_message(AppMsg::Error("Не авторизован".into()));
                }
                false
            }
            AppMsg::UpdatePost(id, title, content) => {
                if let Some(token) = &self.token {
                    let link = ctx.link().clone();
                    let token = token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::update_post(&token, id, title, content).await {
                            Ok(_) => {
                                link.send_message(AppMsg::LoadPosts);
                                link.send_message(AppMsg::ClearEditing);
                            }
                            Err(e) => link.send_message(AppMsg::Error(e.to_string())),
                        }
                    });
                } else {
                    ctx.link()
                        .send_message(AppMsg::Error("Не авторизован".into()));
                }
                false
            }
            AppMsg::DeletePost(id) => {
                if let Some(token) = &self.token {
                    let link = ctx.link().clone();
                    let token = token.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match api::delete_post(&token, id).await {
                            Ok(_) => link.send_message(AppMsg::LoadPosts),
                            Err(e) => link.send_message(AppMsg::Error(e.to_string())),
                        }
                    });
                } else {
                    ctx.link()
                        .send_message(AppMsg::Error("Не авторизован".into()));
                }
                false
            }
            AppMsg::EditPost(post) => {
                self.editing_post = Some(post);
                true
            }
            AppMsg::ClearEditing => {
                self.editing_post = None;
                true
            }
            AppMsg::Error(err) => {
                self.error = Some(err);
                self.success = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let error_html = if let Some(err) = &self.error {
            html! { <div class="error">{ err }</div> }
        } else {
            html! {}
        };
        let success_html = if let Some(msg) = &self.success {
            html! { <div class="success">{ msg }</div> }
        } else {
            html! {}
        };

        // Если пользователь не залогинен, показываем формы входа/регистрации
        if self.current_user.is_none() {
            let on_login_success = ctx.link().callback(AppMsg::LoginSuccess);
            let on_register_success = ctx.link().callback(AppMsg::RegisterSuccess);
            let on_error = ctx.link().callback(AppMsg::Error);
            return html! {
                <div>
                    { error_html }
                    { success_html }
                    <LoginForm on_login_success={on_login_success} on_error={on_error.clone()} />
                    <hr />
                    <RegisterForm {on_register_success} {on_error} />
                </div>
            };
        }

        // Пользователь залогинен
        let user = self.current_user.as_ref().unwrap();
        let logout_cb = ctx.link().callback(|_| AppMsg::Logout);
        let create_cb = ctx
            .link()
            .callback(|(title, content)| AppMsg::CreatePost(title, content));
        let edit_cb = ctx.link().callback(|post| AppMsg::EditPost(post));
        let delete_cb = ctx.link().callback(|id| AppMsg::DeletePost(id));
        let clear_edit_cb = ctx.link().callback(|_| AppMsg::ClearEditing);
        let update_cb = ctx
            .link()
            .callback(|(id, title, content)| AppMsg::UpdatePost(id, title, content));

        let editing_form = if let Some(post) = &self.editing_post {
            let post_id = post.id.clone();
            let post_title = post.title.clone();
            let post_content = post.content.clone();
            html! {
        <div>
            <h3>{"Редактирование поста"}</h3>
            <PostForm
                on_submit={move |(title, content)| {
                    let post_id = post_id.clone(); // клонируем перед использованием
                    update_cb.emit((post_id, title, content))
                }}
                button_label="Сохранить изменения"
                initial_title={Some(post_title)}
                initial_content={Some(post_content)}
            />
            <button onclick={clear_edit_cb}>{"Отмена"}</button>
            <hr />
        </div>
    }
        } else {
            html! {}
        };

        html! {
            <div>
                <div style="display: flex; justify-content: space-between; align-items: center;">
                    <h2>{ format!("Добро пожаловать, {}!", user.username) }</h2>
                    <button onclick={logout_cb}>{"Выйти"}</button>
                </div>
                { error_html }
                { success_html }
                <hr />
                <h3>{"Создать новый пост"}</h3>
                <PostForm
                    on_submit={create_cb}
                    button_label="Опубликовать"
                    initial_title={None::<String>}
                    initial_content={None::<String>}
                />
                <hr />
                <h3>{"Все посты"}</h3>
                { editing_form }
                <PostList
                    posts={self.posts.clone()}
                    current_user_id={Some(user.id.clone())}
                    on_edit={edit_cb}
                    on_delete={delete_cb}
                />
            </div>
        }
    }
}
