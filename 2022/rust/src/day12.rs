use crate::LINE_END;
use pathfinding::prelude::bfs;

#[derive(Debug)]
struct Heightmap {
    width: usize,
    height: usize,
    data: Vec<char>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i64, i64);

impl Heightmap {
    fn from_input(input: &str, width: usize) -> Self {
        let data: Vec<char> = input
            .split(LINE_END)
            .flat_map(|line| line.trim().chars())
            .collect();
        Self {
            width,
            height: data.len() / width,
            data,
        }
    }

    fn find_pos(&self, character: char) -> Pos {
        let (idx, _) = self
            .data
            .iter()
            .enumerate()
            .find(|(_, c)| **c == character)
            .unwrap();

        self.index_to_pos(idx)
    }

    fn find_all_pos(&self, character: char) -> Vec<Pos> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == character)
            .map(|(index, _)| self.index_to_pos(index))
            .collect()
    }

    fn index_to_pos(&self, index: usize) -> Pos {
        Pos((index % self.width) as i64, (index / self.width) as i64)
    }

    fn pos_to_index(&self, Pos(x, y): Pos) -> usize {
        y as usize * self.width + x as usize
    }

    fn elevation_at(&self, pos: Pos) -> char {
        match self.data[self.pos_to_index(pos)] {
            'S' => 'a',
            'E' => 'z',
            c => c,
        }
    }

    fn can_move_to(&self, from: Pos, to: Pos) -> bool {
        let a = self.elevation_at(from);
        let b = self.elevation_at(to);
        b as i8 - a as i8 <= 1
    }

    fn successors(&self, pos: &Pos) -> Vec<Pos> {
        let &Pos(x, y) = pos;
        let all = vec![Pos(x - 1, y), Pos(x + 1, y), Pos(x, y - 1), Pos(x, y + 1)];

        let c = self.data[self.pos_to_index(*pos)];

        all.iter()
            .filter(|Pos(x, y)| {
                *x >= 0
                    && *x < self.width as i64
                    && *y >= 0
                    && *y < self.height as i64
                    && (c == 'S' || self.can_move_to(*pos, Pos(*x, *y)))
            })
            .copied()
            .collect()
    }
}

pub fn part01(input: &str, width: usize) -> usize {
    // parse into a grid
    let heightmap = Heightmap::from_input(input, width);

    // find start and end point
    let start = heightmap.find_pos('S');
    let goal = heightmap.find_pos('E');

    // A*
    bfs(&start, |p| heightmap.successors(p), |p| *p == goal)
        .unwrap()
        .len()
        - 1
}

pub fn part02(input: &str, width: usize) -> usize {
    let heightmap = Heightmap::from_input(input, width);

    let mut start_positions = heightmap.find_all_pos('a');
    start_positions.push(heightmap.find_pos('S'));
    let goal = heightmap.find_pos('E');

    let mut t: Vec<_> = start_positions
        .iter()
        .filter_map(|pos| bfs(pos, |p| heightmap.successors(p), |p| *p == goal))
        .map(|l| l.len() - 1)
        .collect();
    t.sort();
    t[0]
}

#[cfg(test)]
mod tests {
    use crate::{
        day12::{part01, part02},
        input,
    };

    const HEIGHTMAP: &str = "\
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    ";

    #[test]
    fn day_12_part_01() {
        assert_eq!(part01(HEIGHTMAP, 8), 31);
        assert_eq!(part01(input("day_12").as_str(), 143), 462);
    }

    #[test]
    fn day_12_part_02() {
        assert_eq!(part02(HEIGHTMAP, 8), 29);
        assert_eq!(part02(input("day_12").as_str(), 143), 451);
    }
}
