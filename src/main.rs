#![feature(array_windows)]

use grid::Grid;
use yew::prelude::*;

mod grid;

enum Msg {
    Start,
    Move(usize),
}

struct App {
    moves: u32,
    grid: Grid,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { grid: Grid::new(4), moves: 0 }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.grid.shuffle();
            }
            Msg::Move(i) => {
                self.grid.r#move(i);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let g = &self.grid;
        let tiles: Html = g.value_sorted_pairs().iter().map(|&(i, n)| {
            let x = i % g.size();
            let y = i / g.size();
            html! {
                <div
                    key={n.to_string()}
                    data-n={n.to_string()}
                    class="tile"
                    // style={ format!("transform: translate({}%, {}%); transition: transform ease 0.2s", x*100, y*100) }
                    style={ format!("--x: {}; --y: {}; ", x, y) }
                    onclick={ctx.link().callback(move |_| Msg::Move(i))}
                >
                    { n.to_string() }
                </div>
            }
        }).collect();

        return html! {
            <div>
                <header>
                    <h1>{ "15" }</h1>
                    // <label>
                    //     { "Size: " }
                    //     <select>
                    //         <option value=3>{ "3x3" }</option>
                    //         <option value=4 selected=true>{ "4x4" }</option>
                    //         <option value=5>{ "5x5" }</option>
                    //         <option value=6>{ "6x6" }</option>
                    //         <option value=7>{ "7x7" }</option>
                    //         <option value=8>{ "8x8" }</option>
                    //     </select>
                    // </label>
                    // {"Moves: "} {self.moves}
                    <button onclick={ctx.link().callback(|_| Msg::Start)}>{ "Start" }</button>
                </header>
                <div class="grid" style="">{ tiles }</div>
            </div>
        };
    }
}

fn main() {
    yew::start_app::<App>();
}
