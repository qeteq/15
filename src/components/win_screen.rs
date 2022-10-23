use yew::{function_component, html, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub moves: u32,
    pub onback: Callback<()>,
}

#[function_component(WinScreen)]
pub fn win_screen(props: &Props) -> Html {
    html! {
        <div class="game game-win">
            <h1>{ format!("You won after {} moves!", props.moves) }</h1>
            <button onclick={props.onback.reform(|_| {})}>{ "Back to menu" }</button>
        </div>
    }
}
