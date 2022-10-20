#![feature(array_windows)]

use grid::Grid;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

mod grid;
mod ui;

enum Msg {
    Start,
    Move(usize),
    ChangeSize(usize),
}

struct App {
    moves: u32,
    grid: Grid,
    msg: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let mut grid = Grid::new(4);
        grid.shuffle();
        Self { grid, moves: 0, msg: "".into() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.moves = 0;
                self.grid.shuffle();
            }
            Msg::Move(i) => {
                if self.grid.r#move(i) {
                    self.moves += 1;
                }
            }
            Msg::ChangeSize(size) => {
                self.grid = Grid::new(size);
                self.grid.shuffle();
                self.moves = 0;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let g = &self.grid;
        let link = ctx.link();

        let l = link.clone();
        let handle_size_change: Callback<Event> = (move |e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let parsed = target.value().parse::<usize>();

            if let Ok(size) = parsed {
                l.send_message(Msg::ChangeSize(size));
            }
        }).into();

        let handle_start_click = ctx.link().callback(|_| Msg::Start);

        return html! {
            <div>
                <header>
                    <h1>{ "15" }</h1>
                    <label>
                        { "Size: " }
                        <select onchange={handle_size_change}>
                            <option value=3>{ "3x3" }</option>
                            <option value=4 selected=true>{ "4x4" }</option>
                            <option value=5>{ "5x5" }</option>
                            <option value=6>{ "6x6" }</option>
                            <option value=7>{ "7x7" }</option>
                            <option value=8>{ "8x8" }</option>
                        </select>
                    </label>
                    <button onclick={handle_start_click}>{ "Start" }</button>
                    {"Moves: "} {self.moves}
                </header>
                <ui::grid::Grid
                    size={self.grid.size()}
                    tiles={Box::new(self.grid.tiles().clone())}
                    on_tile_click={ctx.link().callback(|i| Msg::Move(i))}
                />
            </div>
        };
    }
}

fn main() {
    yew::start_app::<App>();
}
