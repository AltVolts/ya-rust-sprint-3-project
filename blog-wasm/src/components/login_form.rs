use yew::prelude::*;
use crate::api;
use crate::types::User;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_login_success: Callback<User>,
    pub on_error: Callback<String>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &Props) -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let loading = use_state(|| false);

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let loading = loading.clone();
        let on_success = props.on_login_success.clone();
        let on_error = props.on_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *loading {
                return;
            }
            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let loading = loading.clone();
            let on_success = on_success.clone();
            let on_error = on_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match api::login(username_val, password_val).await {
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
            <h2>{"Вход"}</h2>
            <div class="form-group">
                <label>{"Имя пользователя"}</label>
                <input
                    type="text"
                    value={(*username).clone()}
                    oninput={move |e: InputEvent| {
                        let input = e.target_unchecked_into::<HtmlInputElement>();
                        username.set(input.value());
                    }}
                    required={true}
                />
            </div>
            <div class="form-group">
                <label>{"Пароль"}</label>
                <input
                    type="password"
                    value={(*password).clone()}
                    oninput={move |e: InputEvent| {
                        let input = e.target_unchecked_into::<HtmlInputElement>();
                        password.set(input.value());
                    }}
                    required={true}
                />
            </div>
            <button type="submit" disabled={*loading}>
                { if *loading { "Вход..." } else { "Войти" } }
            </button>
        </form>
    }
}