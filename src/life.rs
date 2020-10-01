use rand;
use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Life {
    Alive,
    Dead,
}

impl Life {
    pub fn advance(self, neighbours: usize) -> Self {
        match self {
            Life::Alive if (neighbours == 2) || (neighbours == 3) => Life::Alive,
            Life::Dead if neighbours == 3 => Life::Alive,
            _ => Life::Dead,
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 2) {
            0 => Life::Alive,
            _ => Life::Dead,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

const DIFFS: &'static [Point; 8] = &[
    Point { x: -1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: -1 },
];

pub type Game = HashMap<Point, Life>;

pub fn new_game(width: u16, height: u16) -> Game {
    let mut res = HashMap::new();
    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
             res.insert(
                 Point {
                     x: x as i32,
                     y: y as i32,
                 },
                 Life::Dead,
             );
        });
    });
    res
}

pub fn neighbours(g: &Game, target: &Point) -> usize {
    DIFFS
        .into_iter()
        .map(|diff| *diff + *target)
        .map(|point| g.get(&point))
        .filter(|&neighbour| match neighbour {
            Some(Life::Alive) => true,
            _ => false,
        })
        .count()
}

pub fn randomize(game: &mut Game) {
    game.values_mut().for_each(|v| {
        *v = Life::random();
    })
}

pub fn advance(game: &mut Game) -> Game {
    let tmp = game.clone();
    game.into_iter().for_each(|(k, v)| {
        let n = neighbours(&tmp, &k);
        *v = v.advance(n);
    });
    tmp
}

pub fn show(game: &Game, width: u16, height: u16) -> String {
    let mut res = String::new();
    (0..height).rev().for_each(|y| {
        (0..width).for_each(|x| {
            let c = {
                match game.get(&Point { x: x as i32, y: y as i32 }) {
                    Some(Life::Alive) => '*',
                    Some(Life::Dead) => ' ',
                    _ => '?',
                }
            };
            res.push(c);
        });
        res.push('\n');
    });
    res
}
