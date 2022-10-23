use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub size: usize,
    pub onsizechange: Callback<usize>,
    pub onstart: Callback<()>,
}

#[function_component(MenuScreen)]
pub fn menu_screen(props: &Props) -> Html {
    let inc = {
        let size = props.size.clone();
        props.onsizechange.reform(move |_| size + 1)
    };
    let dec = {
        let size = props.size.clone();
        props.onsizechange.reform(move |_| size - 1)
    };

    html! {
        <div class="game game-menu">
            <label>
                {"Size: "}
                <button class="resize" type="button" onclick={dec}>{"<"}</button>
                <span class="size">{props.size}</span>
                <button class="resize" type="button" onclick={inc}>{">"}</button>
            </label>
            <button class="start" onclick={props.onstart.reform(|_| {})}>{ "Start" }</button>
        </div>
    }
}
