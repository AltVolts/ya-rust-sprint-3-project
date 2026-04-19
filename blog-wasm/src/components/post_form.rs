use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_submit: Callback<(String, String)>,
    pub button_label: String,
    pub initial_title: Option<String>,
    pub initial_content: Option<String>,
}

#[function_component(PostForm)]
pub fn post_form(props: &Props) -> Html {
    let title = use_state(|| props.initial_title.clone().unwrap_or_default());
    let content = use_state(|| props.initial_content.clone().unwrap_or_default());
    let loading = use_state(|| false);

    let on_submit = {
        let title = title.clone();
        let content = content.clone();
        let loading = loading.clone();
        let on_submit = props.on_submit.clone();
        let _button_label = props.button_label.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if *loading {
                return;
            }
            let title_val = (*title).clone();
            let content_val = (*content).clone();
            let loading = loading.clone();
            let on_submit = on_submit.clone();
            if title_val.trim().is_empty() || content_val.trim().is_empty() {
                return;
            }
            loading.set(true);
            on_submit.emit((title_val, content_val));
            // Сброс формы после успешной отправки будет в родителе
            loading.set(false);
            title.set(String::new());
            content.set(String::new());
        })
    };

    html! {
        <form onsubmit={on_submit}>
            <div class="form-group">
                <label>{"Заголовок"}</label>
                <input type="text" value={(*title).clone()} oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    title.set(input.value());
                }} required={true}
            />
            </div>
            <div class="form-group">
                <label>{"Содержание"}</label>
                <textarea rows="5" value={(*content).clone()} oninput={move |e: InputEvent| {
                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                    content.set(input.value());
                }} required={true}
                />
            </div>
            <button type="submit" disabled={*loading}>{ &props.button_label }</button>
        </form>
    }
}
