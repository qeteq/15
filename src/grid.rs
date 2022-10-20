use std::{ops::RangeInclusive, cmp::Ordering};

pub struct Grid {
    size: usize,
    tiles: Vec<usize>,
}

fn range_inclusive(from: usize, to: usize) -> Vec<usize> {
    if from < to {
        (from..=to).collect()
    } else {
        (to..=from).rev().collect()
    }
}

impl Grid {
    pub fn new(size: usize) -> Grid {
        let tiles: Vec<usize> = vec![0; size * size];
        let mut grid = Grid { size, tiles };
        grid.reset_tiles();
        return grid;
    }

    pub fn tiles(&self) -> &Vec<usize> {
        self.tiles.as_ref()
    }

    pub fn value_sorted_pairs(&self) -> Vec<(usize, &usize)> {
        let mut pairs: Vec<_> = self.tiles.iter().enumerate().collect();
        pairs.sort_by(|&(_, a), &(_, b)| {
            if a == b { return Ordering::Equal }
            if *a == 0 { return Ordering::Greater }
            if *b == 0 { return Ordering::Less }
            return a.cmp(b);
        });
        return pairs;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn reset_tiles(&mut self) {
        let len = self.tiles.len();
        for i in 1..len {
            self.tiles[i - 1] = i;
        }
        self.tiles[len - 1] = 0;
    }

    pub fn shuffle(&mut self) {
        let len = self.tiles.len();
        self.reset_tiles();
        let mut swaps = 0;
        while swaps < len * 50 {
            let i = rand::random::<usize>() % (len - 1);
            let j = rand::random::<usize>() % (len - 1);
            if self.swap(i, j) {
                swaps += 1;
            };
        }
    }

    fn index_position(&self, index: usize) -> (usize, usize) {
        let x = index % self.size;
        let y = index / self.size;
        return (x, y);
    }

    fn zero_position(&self) -> (usize, usize) {
        let mut zero_index: usize = 0;

        for i in 0..self.tiles.len() {
            if self.tiles[i] == 0 {
                zero_index = i;
            }
        }
        return self.index_position(zero_index);
    }

    pub fn move_by_pos(&mut self, x: usize, y: usize) -> bool {
        return self.r#move(x + y * self.size);
    }

    pub fn r#move(&mut self, from: usize) -> bool {
        let len = self.tiles.len();

        if from >= len || self.tiles[from] == 0 {
            return false;
        }

        let from_pos = self.index_position(from);
        let zero_pos = self.zero_position();
        if from_pos.0 != zero_pos.0 && from_pos.1 != zero_pos.1 {
            return false;
        }
        if from_pos.0 == zero_pos.0 && from_pos.1 == zero_pos.1 {
            return false;
        }

        if from_pos.0 == zero_pos.0 {
            // same column
            let x = from_pos.0;
            let range: Vec<_> = range_inclusive(zero_pos.1, from_pos.1);
            for [prev, next] in range.array_windows() {
                self.swap_by_pos((x, *prev), (x, *next));
            }
        } else {
            // same row
            let y = from_pos.1;
            let range: Vec<_> = range_inclusive(zero_pos.0, from_pos.0);
            for [prev, next] in range.array_windows() {
                self.swap_by_pos((*prev, y), (*next, y));
            }
        }

        return true;
    }

    fn swap_by_pos(&mut self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        let i = pos1.0 + pos1.1 * self.size;
        let j = pos2.0 + pos2.1 * self.size;
        return self.swap(i, j);
    }

    fn swap(&mut self, i: usize, j: usize) -> bool {
        if i == j || i > self.tiles.len() || j > self.tiles.len() {
            return false;
        }
        let v = self.tiles[i];
        self.tiles[i] = self.tiles[j];
        self.tiles[j] = v;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn test() {
        let xy = 1..10;
        let x: Vec<_> = (10..=1).collect();
        println!("x = {:?}", x);
    }

    #[test]
    fn build_1() {
        let g = Grid::new(1);
        assert_eq!(g.size, 1);
        assert_eq!(g.tiles, vec![0]);
    }

    #[test]
    fn build_4() {
        let g = Grid::new(4);
        assert_eq!(g.size, 4);
        assert_eq!(
            g.tiles,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
        );
    }

    #[test]
    fn build_7() {
        let g = Grid::new(7);
        assert_eq!(g.size, 7);
        let mut expected: Vec<usize> = (1..=48).collect();
        expected.push(0);
        assert_eq!(g.tiles, expected);
    }

    #[test]
    fn shuffle_shuffles_tiles() {
        let mut g = Grid::new(4);

        g.shuffle();
        let tiles_1 = g.tiles.clone();
        g.shuffle();
        let tiles_2 = g.tiles;

        assert_ne!(tiles_1, tiles_2);
    }

    #[test]
    fn shuffle_ensures_last_tile_is_0() {
        let mut g = Grid::new(4);
        g.shuffle();
        assert_eq!(g.tiles[g.tiles.len() - 1], 0);

        g.shuffle();
        assert_eq!(g.tiles[g.tiles.len() - 1], 0);
    }

    #[test]
    fn index_position_is_correct() {
        let g = Grid::new(4);

        assert_eq!(g.index_position(0), (0, 0));
        assert_eq!(g.index_position(1), (1, 0));
        assert_eq!(g.index_position(2), (2, 0));
        assert_eq!(g.index_position(3), (3, 0));
        assert_eq!(g.index_position(4), (0, 1));
        assert_eq!(g.index_position(9), (1, 2));
        assert_eq!(g.index_position(15), (3, 3));
    }

    #[test]
    fn valid_swap() {
        let mut g = Grid::new(3);

        assert!(g.swap(0, 1));
        //                       x  x
        assert_eq!(g.tiles, vec![2, 1, 3, 4, 5, 6, 7, 8, 0]);

        assert!(g.swap(1, 2));
        //                          x  x
        assert_eq!(g.tiles, vec![2, 3, 1, 4, 5, 6, 7, 8, 0]);

        assert!(g.swap(7, 2));
        //                             x              x
        assert_eq!(g.tiles, vec![2, 3, 8, 4, 5, 6, 7, 1, 0]);

        assert!(g.swap(8, 0));
        //                       x                       x
        assert_eq!(g.tiles, vec![0, 3, 8, 4, 5, 6, 7, 1, 2]);
    }

    #[test]
    fn invalid_swap_does_not_modify_tiles() {
        let mut g = Grid::new(3);
        let out_of_bounds = 90;
        let out_of_bounds_2 = out_of_bounds + 1;
        let initial_tiles: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert!(!g.swap(0, out_of_bounds));
        assert_eq!(g.tiles, initial_tiles);

        assert!(!g.swap(out_of_bounds, 0));
        assert_eq!(g.tiles, initial_tiles);

        assert!(!g.swap(out_of_bounds, out_of_bounds_2));
        assert_eq!(g.tiles, initial_tiles);

        assert!(!g.swap(1, 1));
        assert_eq!(g.tiles, initial_tiles);
    }

    #[test]
    fn move_1_tile() {
        let mut g = Grid::new(3);

        assert!(g.r#move(5));
        assert_eq!(g.tiles, vec![1, 2, 3, 4, 5, 0, 7, 8, 6]);

        assert!(g.r#move(4));
        assert_eq!(g.tiles, vec![1, 2, 3, 4, 0, 5, 7, 8, 6]);

        assert!(g.r#move(7));
        assert_eq!(g.tiles, vec![1, 2, 3, 4, 8, 5, 7, 0, 6]);

        assert!(g.r#move(8));
        assert_eq!(g.tiles, vec![1, 2, 3, 4, 8, 5, 7, 6, 0]);
    }

    #[test]
    fn move_many_tiles() {
        let mut g = Grid::new(4);

        assert!(g.r#move(3));
        assert_eq!(
            g.tiles,
            vec![1, 2, 3, 0, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12]
        );

        assert!(g.r#move(0));
        assert_eq!(
            g.tiles,
            vec![0, 1, 2, 3, 5, 6, 7, 4, 9, 10, 11, 8, 13, 14, 15, 12]
        );

        assert!(g.r#move(12));
        assert_eq!(
            g.tiles,
            vec![5, 1, 2, 3, 9, 6, 7, 4, 13, 10, 11, 8, 0, 14, 15, 12]
        );

        assert!(g.r#move(15));
        assert_eq!(
            g.tiles,
            vec![5, 1, 2, 3, 9, 6, 7, 4, 13, 10, 11, 8, 14, 15, 12, 0]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn move_by_pos() {
        let mut g = Grid::new(4);

        assert!(g.move_by_pos(3, 0));
        assert_eq!(g.tiles, vec![
            1,  2,  3,  0,
            5,  6,  7,  4,
            9,  10, 11, 8,
            13, 14, 15, 12
        ]);

        assert!(g.move_by_pos(0, 0));
        assert_eq!(g.tiles, vec![
            0,  1,  2,  3,
            5,  6,  7,  4,
            9,  10, 11, 8,
            13, 14, 15, 12
        ]);

        assert!(g.move_by_pos(0, 3));
        assert_eq!(g.tiles, vec![
            5,  1,  2,  3,
            9,  6,  7,  4,
            13, 10, 11, 8,
            0,  14, 15, 12
        ]);

        assert!(g.move_by_pos(3, 3));
        assert_eq!(g.tiles, vec![
            5,  1,  2,  3,
            9,  6,  7,  4,
            13, 10, 11, 8,
            14, 15, 12, 0
        ]);
    }

    #[test]
    fn invalid_move_does_not_modify_tiles() {
        let mut g = Grid::new(3);
        let original_tiles = g.tiles.clone();

        assert!(!g.move_by_pos(0, 0));
        assert_eq!(g.tiles, original_tiles);

        assert!(!g.move_by_pos(1, 0));
        assert_eq!(g.tiles, original_tiles);

        assert!(!g.move_by_pos(0, 1));
        assert_eq!(g.tiles, original_tiles);

        assert!(!g.move_by_pos(1, 1));
        assert_eq!(g.tiles, original_tiles);
    }

    #[test]
    fn value_sorted_pairs() {
        let mut g = Grid::new(3);
        assert_eq!(g.value_sorted_pairs(), vec![
            (0, &1), (1, &2), (2, &3),
            (3, &4), (4, &5), (5, &6),
            (6, &7), (7, &8), (8, &0)
        ]);

        g.r#move(2);
        g.r#move(1);

        assert_eq!(g.value_sorted_pairs(), vec![
            (0, &1), (2, &2), (5, &3),
            (3, &4), (4, &5), (8, &6),
            (6, &7), (7, &8), (1, &0)
        ]);
    }
}