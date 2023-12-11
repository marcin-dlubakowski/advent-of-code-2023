type Tile = (bool, u64, u64);

fn parse_data(expansion_factor: u64) -> Vec<Vec<Tile>> {
    let mut data: Vec<Vec<Tile>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .enumerate()
        .map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(|(y, c)| (c == '#', x as u64, y as u64))
                .collect::<Vec<_>>()
        })
        .collect();

    // Expand rows
    let mut cur_x = 0;
    for x in 0..data.len() {
        if data[x].iter().all(|el| !el.0) {
            cur_x += expansion_factor;
        } else {
            cur_x += 1;
        }
        for y in 0..data[0].len() {
            let (val, _, tile_y) = data[x][y];
            data[x][y] = (val, cur_x - 1, tile_y);
        }
    }
    // Expand cols
    let mut cur_y = 0;
    for y in 0..data[0].len() {
        if data.iter().all(|row| !row[y].0) {
            cur_y += expansion_factor;
        } else {
            cur_y += 1;
        }
        for x in 0..data.len() {
            let (val, tile_x, _) = data[x][y];
            data[x][y] = (val, tile_x, cur_y - 1);
        }
    }

    data
}

fn solve(expansion_factor: u64) {
    let data = parse_data(expansion_factor);
    let coords: Vec<(u64, u64)> = data
        .iter()
        .flat_map(|row| {
            row.iter()
                .flat_map(|tile| {
                    if tile.0 {
                        vec![(tile.1, tile.2)]
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result = 0;

    for a in 0..coords.len() {
        for b in (a + 1)..coords.len() {
            let (x1, y1) = coords[a];
            let (x2, y2) = coords[b];
            result += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }

    println!("Result is : {result}");
}

fn main() {
    solve(2);
    solve(10);
    solve(100);
    solve(1000000);
}
