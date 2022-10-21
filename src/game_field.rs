use std::cmp::Ordering;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub size: usize,
    pub tiles: Box<Vec<usize>>,
    pub on_tile_click: Callback<usize>,
}

fn sorted_tiles_by_value<'a>(tiles: &'a Vec<usize>) -> Vec<(usize, &'a usize)> {
    let mut pairs: Vec<_> = tiles.iter().enumerate().collect();
    pairs.sort_by(|&(_, a), &(_, b)| {
        if a == b {
            return Ordering::Equal;
        }
        if *a == 0 {
            return Ordering::Greater;
        }
        if *b == 0 {
            return Ordering::Less;
        }
        return a.cmp(b);
    });
    return pairs;
}

#[function_component(GameField)]
pub fn game_field(props: &Props) -> Html {
    let pairs = sorted_tiles_by_value(&props.tiles);
    let size = props.size;
    let tiles: Html = pairs
        .iter()
        .map(|&(i, n)| {
            let x = i % size;
            let y = i / size;
            let nx = match n {
                0 => size,
                n => (n - 1) % size,
            };
            let ny = match n {
                0 => size,
                n => (n - 1) / size,
            };
            html! {
                <div
                    key={format!("{}-{}", size, n)}
                    data-n={n.to_string()}
                    class="tile"
                    style={ format!("--x: {}; --y: {}; --nx: {}; --ny: {}", x, y, nx, ny) }
                    onclick={props.on_tile_click.reform(move |_| i)}
                >
                    { n.to_string() }
                </div>
            }
        })
        .collect();

    return html! {
        <div class="grid" style={format!("--s: {}", size)}>{ tiles }</div>
    };
}
