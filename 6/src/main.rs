fn solution(races: &[(i64, i64)]) {
    let result = races
        .iter()
        .map(|(max_time, record)| {
            let times: Vec<i64> = (1..*max_time)
                .map(|time| time * (max_time - time))
                .filter(|time| time > record)
                .collect();
            times.len() as i64
        })
        .product::<i64>();

    println!("Result is: #{result}");
}

fn main() {
    solution(&[(61, 430), (67, 1036), (75, 1307), (71, 1150)]);
    solution(&[(61677571, 430103613071150)]);
}
