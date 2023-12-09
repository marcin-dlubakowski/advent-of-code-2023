fn parse_data() -> Vec<Vec<i32>> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect()
}

fn extrapolate(datapoints: &[i32]) -> i32 {
    if datapoints.iter().all(|val| *val == 0) {
        return 0;
    }

    let last = datapoints.last().unwrap();
    let diffs: Vec<i32> = datapoints
        .windows(2)
        .map(|vals| vals[1] - vals[0])
        .collect();

    last + extrapolate(&diffs)
}

fn solve(reverse: bool) {
    let data = parse_data();
    let result: i32 = data
        .into_iter()
        .map(|mut datapoints| {
            if reverse {
                datapoints.reverse();
            }
            extrapolate(&datapoints)
        })
        .sum();

    println!("Result is: {result}");
}

fn main() {
    solve(false);
    solve(true);
}
