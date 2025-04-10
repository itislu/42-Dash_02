use std::{env, fs::read_to_string};

#[derive(Debug)]
enum MapFieldType {
    Node,
    Space(usize),
}

#[derive(Debug)]
struct MapField {
    field_type: MapFieldType,
}

#[derive(Debug)]
struct Map {
    id: usize,
    grid: Vec<Vec<MapField>>,
    height: usize,
    width: usize,
}

fn parse_map(input: &str, id: usize) -> Map {
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '*' => MapField {
                        field_type: MapFieldType::Node,
                    },
                    value => MapField {
                        field_type: MapFieldType::Space((value as u8 - b'0').into()),
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

    println!("{:?}", beacons);
    println!("{:?}", maps);
}
