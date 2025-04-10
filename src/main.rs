use std::{env, fs::read_to_string};

#[derive(Debug, PartialEq, Clone)]
enum MapFieldType {
    Node,
    Space(usize),
}

#[derive(Debug, Clone)]
struct MapField {
    field_type: MapFieldType,
    covered: bool,
}

#[derive(Debug, Clone)]
struct Map {
    id: usize,
    grid: Vec<Vec<MapField>>,
    height: usize,
    width: usize,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

fn parse_map(input: &str, id: usize) -> Map {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '*' => MapField {
                        field_type: MapFieldType::Node,
                        covered: false,
                    },
                    value => MapField {
                        field_type: MapFieldType::Space((value as u8 - b'0').into()),
                        covered: false,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let (height, width) = (grid.len(), grid[0].len());

    Map {
        id,
        grid,
        height,
        width,
    }
}

impl Map {
    fn uncovered_nodes(&self, pos: Position, radius: usize) -> usize {
        let min_col = if radius > pos.col {
            0
        } else {
            pos.col - radius
        };
        let min_row = if radius > pos.row {
            0
        } else {
            pos.row - radius
        };
        let max_col = if pos.col + radius >= self.width {
            self.width - 1
        } else {
            pos.col + radius
        };
        let max_row = if pos.row + radius >= self.height {
            self.height - 1
        } else {
            pos.row + radius
        };

        let mut total = 0;

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if self.grid[row][col].field_type == MapFieldType::Node
                    && !self.grid[row][col].covered
                {
                    total += 1;
                }
            }
        }

        total
    }

    fn put_beacon(&mut self, pos: &Position, radius: usize) {
        let min_col = if radius > pos.col {
            0
        } else {
            pos.col - radius
        };
        let min_row = if radius > pos.row {
            0
        } else {
            pos.row - radius
        };
        let max_col = if pos.col + radius >= self.width {
            self.width - 1
        } else {
            pos.col + radius
        };
        let max_row = if pos.row + radius >= self.height {
            self.height - 1
        } else {
            pos.row + radius
        };

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                if self.grid[row][col].field_type == MapFieldType::Node {
                    self.grid[row][col].covered = true;
                }
            }
        }
    }
}

fn effective_radius(map: &Map, pos: &Position, beacon: usize) -> usize {
    match map.grid[pos.row][pos.col].field_type {
        MapFieldType::Space(value) => value + beacon,
        _ => panic!("Effective radius called on Node"),
    }
}

fn try_map(map: &mut Map, beacons: &[usize]) -> Vec<Position> {
    let mut positions = Vec::new();

    for beacon in beacons {
        let mut max: Vec<(usize, Position)> = Vec::new();
        // Maybe don't start at the border
        for row in 0..map.height {
            for col in 0..map.width {
                if map.grid[row][col].field_type == MapFieldType::Node || map.grid[row][col].covered
                {
                    continue;
                }

                let radius = effective_radius(map, &Position { row, col }, *beacon);
                let value = map.uncovered_nodes(Position { row, col }, radius);
                if max.is_empty() || max[0].0 == value {
                    max.push((value, Position { row, col }));
                } else if value > max[0].0 {
                    max.clear();
                    max.push((value, Position { row, col }));
                }
            }
        }

        // Check the other results too
        let radius = effective_radius(map, &max[0].1, *beacon);
        map.put_beacon(&max[0].1, radius);
        positions.push(max[0].1);
    }

    positions
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let beacons: Vec<_> = args[1]
        .split_whitespace()
        .map(|value| value.parse::<usize>().unwrap())
        .collect();

    let maps: Vec<_> = (2..6)
        .map(|idx| {
            let content = read_to_string(&args[idx]).unwrap();
            parse_map(&content, idx - 2)
        })
        .collect();

    let positions = try_map(&mut maps[0].clone(), &beacons);

    // println!("{:?}", beacons);
    // println!("{:?}", maps);
    println!("{:?}", positions);
}
