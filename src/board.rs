use yew::prelude::*;

use crate::game_field::GameField;
use crate::grid::Grid;

type MovesCount = u32;

pub enum Msg {
    Reset,
    Move(usize),
}

pub struct Board {
    moves: MovesCount,
    grid: Grid,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub size: usize,
    pub onwin: Callback<MovesCount>,
    pub onexit: Callback<()>,
}

impl Component for Board {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            grid: Grid::shuffled(ctx.props().size),
            moves: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.moves = 0;
                self.grid.shuffle();
            }
            Msg::Move(i) => {
                if self.grid.r#move(i) {
                    self.moves += 1;
                }
                if self.grid.is_solved() {
                    ctx.props().onwin.emit(self.moves);
                    self.moves = 0;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_reset_click = ctx.link().callback(|_| Msg::Reset);
        return html! {
            <div>
                <header style="margin-bottom: 20px">
                    {"Moves: "} {self.moves}
                    <button style="margin-left: 20px" onclick={handle_reset_click}>{ "Reset" }</button>
                    <button style="margin-left: 20px" onclick={ctx.props().onexit.reform(|_| {})}>{ "Back to menu" }</button>
                </header>
                <GameField
                    size={self.grid.size()}
                    tiles={Box::new(self.grid.tiles().clone())}
                    on_tile_click={ctx.link().callback(|i| Msg::Move(i))}
                />
            </div>
        };
    }
}
