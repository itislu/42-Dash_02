use std::{cmp::Ordering, env, fs::read_to_string};

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

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.row, self.col)
    }
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
    fn uncovered_nodes(
        &self,
        pos: &Position,
        radius: usize,
        prev_pos: Option<Position>,
        prev_value: Option<usize>,
        prev_radius: Option<usize>,
    ) -> usize {
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

        if let (Some(prev_pos), Some(prev_value), Some(prev_radius)) =
            (prev_pos, prev_value, prev_radius)
        {
            total = prev_value;
            let col_diff = prev_pos.col as isize - pos.col as isize;
            let row_diff = prev_pos.row as isize - pos.row as isize;
            let radius_diff = prev_radius as isize - radius as isize;

            match radius.cmp(&prev_radius) {
                Ordering::Equal => {
                    if col_diff != 0 {
                        for row in min_row..=max_row {
                            if let Some(field) = self.get_by_offset(pos, 0, col_diff) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total -= 1;
                                }
                            }

                            if let Some(field) = self.get_by_offset(pos, 0, -col_diff) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total += 1;
                                }
                            }
                        }
                    } else if row_diff != 0 {
                        for col in min_col..=max_col {
                            if let Some(field) = self.get_by_offset(pos, row_diff, 0) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total -= 1;
                                }
                            }

                            if let Some(field) = self.get_by_offset(pos, -row_diff, 0) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total += 1;
                                }
                            }
                        }
                    }
                }
                Ordering::Less => {
                    for row in (min_row as isize + radius_diff)..=(max_row as isize - radius_diff) {
                        if row < 0 || row as usize >= self.height {
                            continue;
                        }

                        for col in 0..radius_diff.abs() - col_diff {
                            // for col in (min_col - 1 + radius_diff)..=(min_col - 1) {
                            if let Some(field) = self.get_by_offset(
                                &Position {
                                    row: row as usize,
                                    col: min_col,
                                },
                                0,
                                -col + col_diff,
                            ) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total -= 1;
                                }
                            }
                        }

                        for col in 0..radius_diff.abs() + col_diff {
                            if let Some(field) = self.get_by_offset(
                                &Position {
                                    row: row as usize,
                                    col: max_col,
                                },
                                0,
                                col - col_diff,
                            ) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total += 1;
                                }
                            }
                        }
                    }
                    for col in min_col..=max_col {
                        for row in 0..radius_diff.abs() - row_diff {
                            // for col in (min_col - 1 + radius_diff)..=(min_col - 1) {
                            if let Some(field) = self.get_by_offset(
                                &Position {
                                    row: min_row,
                                    col: col as usize,
                                },
                                -row + row_diff,
                                0,
                            ) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total -= 1;
                                }
                            }
                        }

                        for row in 0..radius_diff.abs() + row_diff {
                            if let Some(field) = self.get_by_offset(
                                &Position {
                                    row: max_row,
                                    col: col as usize,
                                },
                                row - row_diff,
                                0,
                            ) {
                                if field.field_type == MapFieldType::Node && !field.covered {
                                    total += 1;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }

            return total;
        }

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

    fn get_by_offset(&self, pos: &Position, row_diff: isize, col_diff: isize) -> Option<&MapField> {
        if (row_diff < 0 && -row_diff as usize > pos.row)
            || (col_diff < 0 && -col_diff as usize > pos.col)
        {
            return None;
        }

        if (row_diff > 0 && row_diff as usize + pos.row >= self.height)
            || (col_diff > 0 && col_diff as usize + pos.col >= self.width)
        {
            return None;
        }

        let (row, col) = (
            (pos.row as isize + row_diff) as usize,
            (pos.col as isize + col_diff) as usize,
        );

        Some(&self.grid[row][col])
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
            let mut prev_pos = None;
            let mut prev_value = None;
            let mut prev_radius = None;

            for col in 0..map.width {
                if map.grid[row][col].field_type == MapFieldType::Node || map.grid[row][col].covered
                {
                    continue;
                }

                let pos = Position { row, col };
                let radius = effective_radius(map, &pos, *beacon);
                let value = map.uncovered_nodes(&pos, radius, prev_pos, prev_value, prev_radius);
                if max.is_empty() || max[0].0 == value {
                    max.push((value, pos.clone()));
                } else if value > max[0].0 {
                    max.clear();
                    max.push((value, pos.clone()));
                }

                prev_pos = Some(pos);
                prev_value = Some(value);
                prev_radius = Some(radius);
            }
        }

        // Check the other results too
        let radius = effective_radius(map, &max[0].1, *beacon);
        map.put_beacon(&max[0].1, radius);
        positions.push(max[0].1);
    }

    positions
}

fn print_result(map: &Map, positions: &[Position]) {
    let mut result = String::new();
    for position in positions {
        result += &format!("{}|", position);
    }
    result.pop();
    println!("{}|{}", map.id, result);
}

fn merge_maps(maps: &[Map], order: &[usize]) -> Map {
    let id = order
        .iter()
        .rev()
        .enumerate()
        .map(|(id, value)| (value + 1) * 10_usize.pow(id as u32))
        .sum();
    println!("{}", id);

    let mut grid = Vec::new();

    for row in 0..maps[order[0]].height {
        let mut line = Vec::new();

        for col in 0..maps[order[0]].width {
            line.push(maps[order[0]].grid[row][col].clone());
        }

        for col in 0..maps[order[1]].width {
            line.push(maps[order[1]].grid[row][col].clone());
        }

        grid.push(line);
    }

    for row in 0..maps[2].height {
        let mut line = Vec::new();

        for col in 0..maps[order[2]].width {
            line.push(maps[order[0]].grid[row][col].clone());
        }

        for col in 0..maps[order[3]].width {
            line.push(maps[order[1]].grid[row][col].clone());
        }

        grid.push(line);
    }

    let (height, width) = (grid.len(), grid[0].len());

    Map {
        id,
        grid,
        height,
        width,
    }
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

    let mut merged = merge_maps(&maps, &[1, 2, 0, 3]);

    let positions = try_map(&mut merged, &beacons);

    print_result(&merged, &positions);
}
