#![feature(array_windows)]
#![feature(is_sorted)]

use board::Board;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod board;
mod game_field;
mod grid;

enum GameStatus {
    InMenu,
    Playing,
    Win,
}

#[function_component(App)]
fn app() -> Html {
    let status = use_state(|| GameStatus::InMenu);
    let game_size = use_state(|| 4);
    let win_moves = use_state(|| 0u32);

    let handle_win = {
        let status = status.clone();
        let win_moves = win_moves.clone();
        Callback::from(move |moves| {
            win_moves.set(moves);
            status.set(GameStatus::Win);
        })
    };

    let handle_start = {
        let status = status.clone();
        Callback::from(move |_| {
            status.set(GameStatus::Playing);
        })
    };

    let handle_goto_menu = {
        let status = status.clone();
        Callback::from(move |_| {
            status.set(GameStatus::InMenu);
        })
    };

    let handle_size_change = {
        let game_size = game_size.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            let parsed = target.value().parse::<usize>();

            if let Ok(mut size) = parsed {
                if size > 8 {
                    size = 8
                }
                if size < 3 {
                    size = 3
                }
                game_size.set(size);
            }
        })
    };

    match *status {
        GameStatus::InMenu => {
            html! {
                <div class="game game-menu">
                    <label>
                        {"Size: "}
                        <input
                            type="number"
                            min="3"
                            max="8"
                            value={(*game_size).to_string()}
                            oninput={handle_size_change}
                        />
                    </label>
                    <button onclick={handle_start}>{ "Start" }</button>
                </div>
            }
        }
        GameStatus::Playing => {
            html! {
                <div class="game game-board">
                    <Board size={*game_size} onwin={handle_win} onexit={handle_goto_menu} />
                </div>
            }
        }
        GameStatus::Win => {
            html! {
                <div class="game game-win">
                    <h1>{ format!("You won after {} moves!", *win_moves) }</h1>
                    <button onclick={handle_goto_menu.reform(|_| {})}>{ "Back to menu" }</button>
                </div>
            }
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
