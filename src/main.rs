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
        prev_value: Option<isize>,
        prev_radius: Option<usize>,
    ) -> isize {
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

        let mut total: isize = 0;

        if let (Some(prev_pos), Some(prev_value), Some(prev_radius)) =
            (prev_pos, prev_value, prev_radius)
        {
            total = prev_value;
            let prev_min_col = if prev_radius > prev_pos.col {
                0
            } else {
                prev_pos.col - prev_radius
            };
            let prev_min_row = if prev_radius > prev_pos.row {
                0
            } else {
                prev_pos.row - prev_radius
            };
            let prev_max_col = if prev_pos.col + prev_radius >= self.width {
                self.width - 1
            } else {
                prev_pos.col + prev_radius
            };
            let prev_max_row = if prev_pos.row + prev_radius >= self.height {
                self.height - 1
            } else {
                prev_pos.row + prev_radius
            };

            // Left side decrease incl. corner
            if min_col > prev_min_col {
                for row in prev_min_row..=prev_max_row {
                    for col in prev_min_col..min_col {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total -= 1;
                        }
                    }
                }
            }
            // Left side increase incl. corner
            else if min_col < prev_min_col {
                for row in min_row..=max_row {
                    for col in min_col..prev_min_col {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total += 1;
                        }
                    }
                }
            }
            // Right side increase incl. corner
            if max_col > prev_max_col {
                for row in min_row..=max_row {
                    for col in prev_max_col + 1..=max_col {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total += 1;
                        }
                    }
                }
            }
            // Right side decrease incl. corner
            else if max_col < prev_max_col {
                for row in prev_min_row..=prev_max_row {
                    for col in max_col + 1..=prev_max_col {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total -= 1;
                        }
                    }
                }
            }

            // Top side decrease
            if min_row > prev_min_row {
                for col in min_col..=max_col {
                    for row in prev_min_row..min_row {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total -= 1;
                        }
                    }
                }
            }
            // Top side increase
            else if min_row < prev_min_row {
                for col in prev_min_col..=prev_max_col {
                    for row in min_row..prev_min_row {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total += 1;
                        }
                    }
                }
            }
            // Bottom side increase
            if max_row > prev_max_row {
                for col in prev_min_col..=prev_max_col {
                    for row in prev_max_row + 1..=max_row {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total += 1;
                        }
                    }
                }
            }
            // Bottom side decrease
            else if max_row < prev_max_row {
                for col in min_col..=max_col {
                    for row in max_row + 1..=prev_max_row {
                        let field = &self.grid[row][col];
                        if field.field_type == MapFieldType::Node && !field.covered {
                            total -= 1;
                        }
                    }
                }
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
        let mut max: Vec<(isize, Position)> = Vec::new();
        // Maybe don't start at the border
        for row in 0..map.height {
            let mut prev_pos = None;
            let mut prev_value = None;
            let mut prev_radius = None;

            if row % 2 == 0 {
                for col in 0..map.width {
                    if map.grid[row][col].field_type == MapFieldType::Node
                        || map.grid[row][col].covered
                    {
                        continue;
                    }

                    let pos = Position { row, col };
                    let radius = effective_radius(map, &pos, *beacon);
                    let value =
                        map.uncovered_nodes(&pos, radius, prev_pos, prev_value, prev_radius);
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
            } else {
                for col in (0..map.width).rev() {
                    if map.grid[row][col].field_type == MapFieldType::Node
                        || map.grid[row][col].covered
                    {
                        continue;
                    }

                    let pos = Position { row, col };
                    let radius = effective_radius(map, &pos, *beacon);
                    let value =
                        map.uncovered_nodes(&pos, radius, prev_pos, prev_value, prev_radius);
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
