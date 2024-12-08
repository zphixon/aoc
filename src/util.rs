use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub fn frequency<K: Eq + Hash>(iter: impl Iterator<Item = K>) -> HashMap<K, u64> {
    let mut counts = HashMap::new();

    for item in iter {
        *counts.entry(item).or_default() += 1;
    }

    counts
}

/// ord doesn't make sense but i need it to put it in btreeset so shrug
#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Direction {
    NW, N, NE,
     W,     E,
    SW, S, SE,
}

impl Direction {
    pub fn to_offset(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            NW => (-1, -1),
            N => (-1, 0),
            NE => (-1, 1),
            W => (0, -1),
            E => (0, 1),
            SW => (1, -1),
            S => (1, 0),
            SE => (1, 1),
        }
    }

    pub fn apply_index<'a, T>(
        &self,
        plane: &'a [Vec<T>],
        row: usize,
        col: usize,
    ) -> Option<(usize, usize)> {
        let (row, col) = (row as isize, col as isize);
        let (row_off, col_off) = self.to_offset();

        let off_row = row + row_off;
        let off_col = col + col_off;

        if in_bounds(plane, off_row, off_col) && !(row_off == 0 && col_off == 0) {
            Some((off_row as usize, off_col as usize))
        } else {
            None
        }
    }

    pub fn apply<'a, T>(&self, plane: &'a [Vec<T>], row: usize, col: usize) -> Option<&'a T> {
        let (new_row, new_col) = self.apply_index(plane, row, col)?;
        Some(&plane[new_row][new_col])
    }

    pub fn right_90(&self) -> Direction {
        use Direction::*;
        match self {
            NW => NE,
            N => E,
            NE => SE,
            W => N,
            E => S,
            SW => NW,
            S => W,
            SE => SW,
        }
    }
}

pub type SurroundingItem<'a, T> = (&'a T, usize, usize, Direction);

pub struct Surrounding<'a, T> {
    plane: &'a [Vec<T>],
    dir: usize,
    row: usize,
    col: usize,
}

impl<'a, T: Debug> Iterator for Surrounding<'a, T> {
    type Item = SurroundingItem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let dirs = [NW, N, NE, W, E, SW, S, SE];
        if self.dir >= dirs.len() {
            tracing::trace!("no more dirs surrounding {},{}", self.row, self.col);
            return None;
        }

        let dir = dirs[self.dir];
        self.dir += 1;

        if let Some((off_row, off_col)) = dir.apply_index(&self.plane, self.row, self.col) {
            tracing::trace!(
                "{:?} {},{} is {:?} of {},{}",
                self.plane[off_row][off_col],
                off_row,
                off_col,
                dir,
                self.row,
                self.col
            );
            Some((&self.plane[off_row][off_col], off_row, off_col, dir))
        } else {
            tracing::trace!("nothing is {:?} of {},{}", dir, self.row, self.col);
            self.next()
        }
    }
}

pub fn surrounding<'a, T: Debug>(
    plane: &'a [Vec<T>],
    row: usize,
    col: usize,
) -> impl Iterator<Item = SurroundingItem<'a, T>> {
    tracing::trace!(
        "look for items surrounding {:?} {},{}",
        plane[row][col],
        row,
        col
    );
    Surrounding {
        plane,
        row,
        col,
        dir: 0,
    }
}

pub fn in_bounds<'a, T>(plane: &'a [Vec<T>], row: isize, col: isize) -> bool {
    (0 <= row && row < plane.len() as isize)
        && (0 <= col && col < plane[row as usize].len() as isize)
}
