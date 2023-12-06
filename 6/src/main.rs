// static RACES: &[(i32, i32)] = &[(7, 9), (15, 40), (30, 200)];
static RACES: &[(i32, i32)] = &[(61, 430), (67, 1036), (75, 1307), (71, 1150)];

fn part_one() {
    let result = RACES
        .iter()
        .map(|(max_time, record)| {
            let times: Vec<i32> = (1..*max_time)
                .map(|time| time * (max_time - time))
                .filter(|time| time > record)
                .collect();
            times.len() as i32
        })
        .product::<i32>();

    println!("Result is: #{result}");
}

fn main() {
    part_one();
}
