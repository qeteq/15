use yew::{html, Callback, Component, Context, Html, Properties};

use crate::board::Board;
use crate::components::board::Board as BoardComponent;

type MovesCount = u32;

pub enum Msg {
    Reset,
    Move(usize),
}

pub struct GameScreen {
    moves: MovesCount,
    board: Board,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub size: usize,
    pub onwin: Callback<MovesCount>,
    pub onexit: Callback<()>,
}

impl Component for GameScreen {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            board: Board::new(ctx.props().size),
            moves: 0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.moves = 0;
                self.board.shuffle();
                true
            }
            Msg::Move(i) => {
                let moved = self.board.r#move(i);
                if moved {
                    if self.board.is_solved() {
                        ctx.props().onwin.emit(self.moves);
                        self.moves = 0;
                    } else {
                        self.moves += 1;
                    }
                }
                moved
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let handle_reset_click = ctx.link().callback(|_| Msg::Reset);

        // TODO: is there a way to avoid cloning?
        let tiles = self.board.tiles().clone();

        return html! {
            <div class="game game-board">
                <header style="margin-bottom: 20px">
                    {"Moves: "} {self.moves}
                    <button style="margin-left: 20px" onclick={handle_reset_click}>{ "Reset" }</button>
                    <button style="margin-left: 20px" onclick={ctx.props().onexit.reform(|_| {})}>{ "Back to menu" }</button>
                </header>
                <BoardComponent
                    size={self.board.size()}
                    tiles={tiles}
                    on_tile_click={ctx.link().callback(|i| Msg::Move(i))}
                />
            </div>
        };
    }
}
