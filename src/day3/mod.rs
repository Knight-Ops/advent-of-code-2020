use fnv::FnvHashMap;
// use std::collections::HashMap;
use std::convert::From;

/// Wrapper type for coordinates (X, Y)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(usize, usize);

/// Wrapper tyep for slope (X, Y) or (Run, Rise)
#[derive(Debug, Clone, Copy)]
pub struct Slope(usize, usize);

/// This represents the two types of tiles we can have, empty or with a tree
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Tree,
}

impl From<char> for Tile {
    fn from(input: char) -> Self {
        match input {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => panic!(format!("Unrecognized character : {}", input)),
        }
    }
}

/// Represents the size of the provided map, we can use this to build an "infinite" representation
/// since the map repeats itself.
#[derive(Debug, Clone)]
pub struct Map {
    height: usize,
    width: usize,
    tiles: FnvHashMap<Coord, Tile>,
}

impl Map {
    /// Converts a &str of '.' and '#' in rows ending with new lines into a `Map`
    /// This assumes that all rows are the same size, and does not check to verify
    pub fn from_str(input: &str) -> Self {
        let mut tiles = FnvHashMap::default();
        let mut height = 0;
        let mut width = 0;
        input.lines().for_each(|line| {
            width = 0;
            line.chars().for_each(|c| {
                let tile = Tile::from(c);
                tiles.insert(Coord(width, height), tile);
                width += 1;
            });
            height += 1;
        });

        Map {
            height,
            width: width,
            tiles: tiles,
        }
    }

    /// Count the trees within the `Map` using a given slope, returning the number of trees
    /// encountered.
    pub fn count_trees_on_slope(&self, input: &Slope) -> usize {
        let mut position = Coord(0, 0);
        let mut trees = 0;

        while position.1 < self.height {
            if self
                .tiles
                .get(&position)
                .expect("Tile is missing from map!")
                == &Tile::Tree
            {
                trees += 1;
            }

            position.0 = (position.0 + input.0) % self.width;
            position.1 += input.1;
        }

        trees
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from_str(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> usize {
    input.count_trees_on_slope(&Slope(3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> usize {
    input.count_trees_on_slope(&Slope(1, 1))
        * input.count_trees_on_slope(&Slope(3, 1))
        * input.count_trees_on_slope(&Slope(5, 1))
        * input.count_trees_on_slope(&Slope(7, 1))
        * input.count_trees_on_slope(&Slope(1, 2))
}
