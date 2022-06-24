#![allow(dead_code)]

use crate::game::Cell;

const A: Cell = Cell::Alive;
const D: Cell = Cell::Dead;


pub const BLINKER: [[Cell; 3]; 3] = [
    [D,A,D],
    [D,A,D],
    [D,A,D],
];

pub const CLOCK: [[Cell; 4]; 4] = [
    [D,A,D,D],
    [D,D,A,A],
    [A,A,D,D],
    [D,D,A,D],
];

pub const PULSAR: [[Cell; 5]; 5] = [
    [D,D,D,D,D],
    [A,D,D,D,A],
    [A,D,A,D,A],
    [A,D,D,D,A],
    [D,D,D,D,D],
];

pub const SPACESHIP:[[Cell; 5]; 4] = [
    [D,A,D,D,A],
    [A,D,D,D,D],
    [A,D,D,D,A],
    [A,A,A,A,D],
];

pub const GLIDER: [[Cell; 3]; 3] = [
    [D,A,A],
    [A,D,A],
    [D,D,A],
];

pub const  PENTADECATHLON:[[Cell; 8]; 3] = [
    [D,A,D,D,D,D,A,D],
    [A,A,D,D,D,D,A,A],
    [D,A,D,D,D,D,A,D],
];

pub const GALAXY: [[Cell; 9]; 9] = [
    [A,A,D,A,A,A,A,A,A],
    [A,A,D,A,A,A,A,A,A],
    [A,A,D,D,D,D,D,D,D],
    [A,A,D,D,D,D,D,A,A],
    [A,A,D,D,D,D,D,A,A],
    [A,A,D,D,D,D,D,A,A],
    [D,D,D,D,D,D,D,A,A],
    [A,A,A,A,A,A,D,A,A],
    [A,A,A,A,A,A,D,A,A],
];

pub const GLIDER_GUN: [[Cell; 38]; 11] = [
    [D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,A,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,A,D,A,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,D,A,A,D,D,D,D,D,D,A,A,D,D,D,D,D,D,D,D,D,D,D,D,A,A,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,A,D,D,D,A,D,D,D,D,A,A,D,D,D,D,D,D,D,D,D,D,D,D,A,A,D],
    [D,A,A,D,D,D,D,D,D,D,D,A,D,D,D,D,D,A,D,D,D,A,A,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,A,A,D,D,D,D,D,D,D,D,A,D,D,D,A,D,A,A,D,D,D,D,A,D,A,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,A,D,D,D,D,D,A,D,D,D,D,D,D,D,A,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,A,D,D,D,A,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,D,A,A,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D],
    [D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D,D],
];

pub const TEST: [[Cell; 3]; 3] = [
    [A,A,A],
    [A,A,A],
    [A,A,A],
];
