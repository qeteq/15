use std::cmp::Ordering;

use yew::prelude::*;


pub struct Grid {
    moves: u32,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub size: usize,
    pub tiles: Box<Vec<usize>>,
    pub on_tile_click: Callback<usize>
}

fn sorted_tiles_by_value<'a>(tiles: &'a Vec<usize>) -> Vec<(usize, &'a usize)> {
    let mut pairs: Vec<_> = tiles.iter().enumerate().collect();
    pairs.sort_by(|&(_, a), &(_, b)| {
        if a == b { return Ordering::Equal }
        if *a == 0 { return Ordering::Greater }
        if *b == 0 { return Ordering::Less }
        return a.cmp(b);
    });
    return pairs;
}

impl Component for Grid {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { moves: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pairs = sorted_tiles_by_value(&ctx.props().tiles);
        let size = ctx.props().size;
        let tiles: Html = pairs.iter().map(|&(i, n)| {
            let x = i % size;
            let y = i / size;
            html! {
                <div
                    key={format!("{}-{}", size, n)}
                    data-n={n.to_string()}
                    class="tile"
                    // style={ format!("transform: translate({}%, {}%); transition: transform ease 0.2s", x*100, y*100) }
                    style={ format!("--x: {}; --y: {}", x, y) }
                    onclick={ctx.props().on_tile_click.reform(move |_| i)}
                >
                    { n.to_string() }
                </div>
            }
        }).collect();

        return html! {
            <div class="grid" style={format!("--s: {}", size)}>{ tiles }</div>
        };
    }
}
