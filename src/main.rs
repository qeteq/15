#![feature(array_windows)]
#![feature(is_sorted)]

use yew::{function_component, html, use_state, Callback};

use components::{game_screen::GameScreen, menu_screen::MenuScreen, win_screen::WinScreen};

mod board;
mod components;

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
        Callback::from(move |newsize: usize| {
            let mut size = newsize;
            if newsize > 8 {
                size = 8
            }
            if newsize < 3 {
                size = 3
            }
            game_size.set(size);
        })
    };

    match *status {
        GameStatus::InMenu => html! {
            <MenuScreen size={*game_size} onstart={handle_start} onsizechange={handle_size_change} />
        },
        GameStatus::Playing => html! {
            <GameScreen size={*game_size} onwin={handle_win} onexit={handle_goto_menu} />
        },
        GameStatus::Win => html! {
            <WinScreen moves={*win_moves} onback={handle_goto_menu} />
        },
    }
}

fn main() {
    yew::start_app::<App>();
}
