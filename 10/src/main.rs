type Pos = (usize, usize);

#[derive(Debug)]
struct Tile {
    l: bool,
    r: bool,
    t: bool,
    b: bool,
}

impl Tile {
    fn new(l: bool, r: bool, t: bool, b: bool) -> Tile {
        Tile { l, r, t, b }
    }

    fn is_start(&self) -> bool {
        self.t && self.b && self.l && self.r
    }
}

fn char_to_tile(c: char) -> Tile {
    match c {
        '.' => Tile::new(false, false, false, false),
        'S' => Tile::new(true, true, true, true),
        '|' => Tile::new(false, false, true, true),
        '-' => Tile::new(true, true, false, false),
        'L' => Tile::new(false, true, true, false),
        'J' => Tile::new(true, false, true, false),
        '7' => Tile::new(true, false, false, true),
        'F' => Tile::new(false, true, false, true),
        _ => panic!("Unrecognized symbol detected"),
    }
}

fn parse_data() -> Vec<Vec<Tile>> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.chars().map(char_to_tile).collect())
        .collect()
}

fn main() {
    let data = parse_data();

    let max_x = data.len();
    let max_y = data[0].len();

    let mut distances_map: Vec<Vec<i32>> = data
        .iter()
        .map(|vec| vec.iter().map(|_| -1i32).collect())
        .collect();

    // Find starting point
    let mut start = (0, 0);
    'outer: for x in 0..max_x {
        for y in 0..max_y {
            if data[x][y].is_start() {
                start = (x, y);
                break 'outer;
            }
        }
    }

    // Calculate distances
    let mut nodes_to_visit = vec![start];
    let mut dist = 1;
    while !nodes_to_visit.is_empty() {
        nodes_to_visit = nodes_to_visit
            .into_iter()
            .flat_map(|node| {
                let (x, y) = node;
                let tile = &data[x][y];
                let mut result: Vec<Pos> = vec![];
                if x > 0 && tile.t && data[x - 1][y].b && distances_map[x - 1][y] == -1 {
                    distances_map[x - 1][y] = dist;
                    result.push((x - 1, y));
                }
                if x < max_x - 1 && tile.b && data[x + 1][y].t && distances_map[x + 1][y] == -1 {
                    distances_map[x + 1][y] = dist;
                    result.push((x + 1, y));
                }
                if y > 0 && tile.l && data[x][y - 1].r && distances_map[x][y - 1] == -1 {
                    distances_map[x][y - 1] = dist;
                    result.push((x, y - 1));
                }
                if y < max_y - 1 && tile.r && data[x][y + 1].l && distances_map[x][y + 1] == -1 {
                    distances_map[x][y + 1] = dist;
                    result.push((x, y + 1));
                }
                result
            })
            .collect();
        dist += 1;
    }

    let result = distances_map.iter().flatten().max().unwrap();

    println!("{data:?}");
    println!("{distances_map:?}");
    println!("Result is: {result}");
}
