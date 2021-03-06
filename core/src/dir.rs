use cgmath::{Vector2};
use position::{MapPos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    SouthEast,
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
}

const DIR_TO_POS_DIFF: [[Vector2<i32>; 6]; 2] = [
    [
        Vector2{x: 1, y: -1},
        Vector2{x: 1, y: 0},
        Vector2{x: 1, y: 1},
        Vector2{x: 0, y: 1},
        Vector2{x: -1, y: 0},
        Vector2{x: 0, y: -1},
    ],
    [
        Vector2{x: 0, y: -1},
        Vector2{x: 1, y: 0},
        Vector2{x: 0, y: 1},
        Vector2{x: -1, y: 1},
        Vector2{x: -1, y: 0},
        Vector2{x: -1, y: -1},
    ]
];

impl Dir {
    pub fn from_int(n: i32) -> Dir {
        assert!(n >= 0 && n < 6);
        let dirs = [
            Dir::SouthEast,
            Dir::East,
            Dir::NorthEast,
            Dir::NorthWest,
            Dir::West,
            Dir::SouthWest,
        ];
        dirs[n as usize]
    }

    pub fn to_int(&self) -> i32 {
        match *self {
            Dir::SouthEast => 0,
            Dir::East => 1,
            Dir::NorthEast => 2,
            Dir::NorthWest => 3,
            Dir::West => 4,
            Dir::SouthWest => 5,
        }
    }

    pub fn get_dir_from_to(from: MapPos, to: MapPos) -> Dir {
        // assert!(from.distance(to) == 1);
        let diff = to.v - from.v;
        let is_odd_row = from.v.y % 2 != 0;
        let subtable_index = if is_odd_row { 1 } else { 0 };
        for dir in dirs() {
            if diff == DIR_TO_POS_DIFF[subtable_index][dir.to_int() as usize] {
                return dir;
            }
        }
        panic!("impossible positions: {}, {}", from, to);
    }

    pub fn get_neighbour_pos(pos: MapPos, dir: Dir) -> MapPos {
        let is_odd_row = pos.v.y % 2 != 0;
        let subtable_index = if is_odd_row { 1 } else { 0 };
        let direction_index = dir.to_int();
        assert!(direction_index >= 0 && direction_index < 6);
        let difference = DIR_TO_POS_DIFF[subtable_index][direction_index as usize];
        MapPos{v: pos.v + difference}
    }
}

#[derive(Clone, Debug)]
pub struct DirIter {
    index: i32,
}

pub fn dirs() -> DirIter {
    DirIter{index: 0}
}

impl Iterator for DirIter {
    type Item = Dir;

    fn next(&mut self) -> Option<Dir> {
        let next_dir = if self.index > 5 {
            None
        } else {
            Some(Dir::from_int(self.index))
        };
        self.index += 1;
        next_dir
    }
}
