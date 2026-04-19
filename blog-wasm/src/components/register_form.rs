use yew::prelude::*;
use crate::api;
use crate::types::User;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_register_success: Callback<User>,
    pub on_error: Callback<String>,
}

#[function_component(RegisterForm)]
pub fn register_form(props: &Props) -> Html {
    let username = use_state(String::new);
    let email = use_state(String::new);
    let password = use_state(String::new);
    let loading = use_state(|| false);

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let loading = loading.clone();
        let on_success = props.on_register_success.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *loading {
                return;
            }
            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let loading = loading.clone();
            let on_success = on_success.clone();
            let on_error = on_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api::register(username_val, email_val, password_val).await {
                    Ok(resp) => {
                        if let Err(e) = api::save_token(&resp.token) {
                            on_error.emit(format!("Ошибка сохранения токена: {}", e));
                        } else {
                            on_success.emit(resp.user);
                        }
                    }
                    Err(e) => on_error.emit(e.to_string()),
                }
                loading.set(false);
            });
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <h2>{"Регистрация"}</h2>
            <div class="form-group">
                <label>{"Имя пользователя"}</label>
                <input type="text" value={(*username).clone()} oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    username.set(input.value());
                }} required={true} />
            </div>
            <div class="form-group">
                <label>{"Email"}</label>
                <input type="email" value={(*email).clone()} oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    email.set(input.value());
                }} required={true} />
            </div>
            <div class="form-group">
                <label>{"Пароль"}</label>
                <input type="password" value={(*password).clone()} oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    password.set(input.value());
                }} required={true} />
            </div>
            <button type="submit" disabled={*loading}>{ if *loading { "Регистрация..." } else { "Зарегистрироваться" } }</button>
        </form>
    }
}