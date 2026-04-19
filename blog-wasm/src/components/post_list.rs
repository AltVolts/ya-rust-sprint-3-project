use yew::prelude::*;
use crate::types::Post;
use chrono::{Local}; // возможно, уже есть импорт; если нет — добавить

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub posts: Vec<Post>,
    pub current_user_id: Option<String>,
    pub on_edit: Callback<Post>,
    pub on_delete: Callback<String>,
}

#[function_component(PostList)]
pub fn post_list(props: &Props) -> Html {
    if props.posts.is_empty() {
        return html! { <p>{"Нет постов. Будьте первым!"}</p> };
    }

    let current_user_id = props.current_user_id.clone();

    html! {
        <>
            {props.posts.iter().map(|post| {
                let is_author = current_user_id.as_ref().map_or(false, |uid| uid == &post.author_id);
                let on_edit = props.on_edit.clone();
                let on_delete = props.on_delete.clone();
                let post_clone = post.clone();
                let post_id = post.id.clone();

                let formatted_date = post.created_at
                    .with_timezone(&Local)
                    .format("%d.%m.%Y %H:%M")
                    .to_string();

                // Сокращаем ID автора до 10 символов
                let short_author_id = if post.author_id.len() >= 8 {
                    format!("{}…", &post.author_id[0..10])
                } else {
                    post.author_id.clone()
                };

                html! {
                    <div key={post.id.clone()} class="post">
                        <h3>{ &post.title }</h3>
                        <div class="post-content">{ &post.content }</div>
                        <div class="post-meta">
                            <span class="post-author">{"Автор: "}{ short_author_id }</span>
                            <span class="post-date">{"📅 "}{ formatted_date }</span>
                        </div>
                        { if is_author {
                            html! {
                                <div class="post-actions">
                                    <button class="btn-edit" onclick={move |_| on_edit.emit(post_clone.clone())}>{"Редактировать"}</button>
                                    <button class="btn-delete" onclick={move |_| on_delete.emit(post_id.clone())}>{"Удалить"}</button>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </div>
                }
            }).collect::<Html>()}
        </>
    }
}