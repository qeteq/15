use std::cmp::Ordering;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct Props {
    pub size: usize,
    pub tiles: Vec<usize>,
    pub on_tile_click: Callback<usize>,
}

fn sorted_tiles_by_number(tiles: &Vec<usize>) -> Vec<(usize, &usize)> {
    let mut pairs: Vec<_> = tiles.iter().enumerate().collect();
    pairs.sort_by(|&(_, a), &(_, b)| {
        if a == b {
            Ordering::Equal
        } else if *a == 0 {
            Ordering::Greater
        } else if *b == 0 {
            Ordering::Less
        } else {
            a.cmp(b)
        }
    });
    return pairs;
}

#[function_component(Board)]
pub fn board(props: &Props) -> Html {
    let pairs = sorted_tiles_by_number(&props.tiles);
    let size = props.size;
    let tiles: Html = pairs
        .into_iter()
        .map(|(i, n)| {
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
