use web_sys::{HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, Properties, TargetCast};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub size: usize,
    pub onsizechange: Callback<usize>,
    pub onstart: Callback<()>,
}

#[function_component(MenuScreen)]
pub fn menu_screen(props: &Props) -> Html {
    let onsizechange = props.onsizechange.to_owned();

    let handle_size_change = {
        let size = props.size.clone();
        Callback::from(move |evt: InputEvent| {
            let input: HtmlInputElement = evt.target_unchecked_into();
            let n = input.value().parse::<usize>().unwrap_or(size);
            onsizechange.emit(n);
        })
    };

    html! {
        <div class="game game-menu">
            <label>
                {"Size: "}
                <input
                    type="number"
                    min="3"
                    max="8"
                    value={props.size.to_string()}
                    oninput={handle_size_change}
                />
            </label>
            <button onclick={props.onstart.reform(|_| {})}>{ "Start" }</button>
        </div>
    }
}
