type Mapping = (i64, i64, i64);
type Area = Vec<Mapping>;

fn parse_input(path: &str) -> (Vec<i64>, Vec<Area>) {
    let data: Vec<Vec<String>> = std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|x| x.trim().split('\n').map(String::from).collect())
        .collect();

    let seeds_data: Vec<i64> = data[0][0]
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let maps_data: Vec<Area> = data
        .into_iter()
        .skip(1)
        .map(|data| {
            data.into_iter()
                .skip(1)
                .map(|line| {
                    let vals: Vec<i64> = line.split(' ').map(|val| val.parse().unwrap()).collect();
                    (vals[0], vals[1], vals[2])
                })
                .collect()
        })
        .collect();

    (seeds_data, maps_data)
}

fn traverse_maps(seed: i64, areas: &[Area]) -> i64 {
    areas.iter().fold(seed, |pos, area| {
        area.iter().fold(pos, |new_pos, mapping| {
            if new_pos != pos {
                return new_pos;
            }
            let (dst, src, range) = *mapping;
            match new_pos {
                x if (src..(src + range)).contains(&x) => dst + (new_pos - src),
                _ => new_pos,
            }
        })
    })
}

fn part_one() {
    let (seeds, areas) = parse_input("input.txt");

    let result = seeds
        .iter()
        .map(|seed| traverse_maps(*seed, &areas))
        .min()
        .unwrap();

    // println!("#{:?}", seeds);
    // println!("#{:?}", areas);
    println!("#{:?}", result);
}

// ALERT: GPT-GENERATED CONTENT
fn set_intersection(set1: (i64, i64), set2: (i64, i64)) -> Option<(i64, i64)> {
    let start = set1.0.max(set2.0);
    let stop = set1.1.min(set2.1);

    if start < stop {
        Some((start, stop))
    } else {
        None
    }
}

fn set_difference(set1: (i64, i64), set2: (i64, i64)) -> Vec<(i64, i64)> {
    let mut result = Vec::new();

    if set2.0 > set1.0 && set2.1 < set1.1 {
        // Case where set2 is fully contained in set1
        result.push((set1.0, set2.0));
        result.push((set2.1, set1.1));
    } else {
        // Cases where sets don't overlap or have partial overlap
        if set1.0 < set2.0 {
            result.push((set1.0, set2.0));
        }
        if set1.1 > set2.1 {
            result.push((set2.1, set1.1));
        }
    }

    result
}
// END ALERT

fn traverse_interval_maps(seed_range: (i64, i64), areas: &[Area]) -> Vec<(i64, i64)> {
    areas.iter().fold(Vec::from(&[seed_range]), |ranges, area| {
        let mut new_ranges = Vec::new();
        let mut leftover_ranges = ranges;
        for mapping in area.iter() {
            println!("Leftover ranges: #{leftover_ranges:?}",);
            println!("New ranges: #{new_ranges:?}");
            let mut new_leftover_ranges = Vec::new();
            for range in leftover_ranges.iter() {
                let src_range = (mapping.1, mapping.1 + mapping.2);
                let movable_range = set_intersection(*range, src_range);
                println!("Source range: #{src_range:?}");
                println!("Current range: #{range:?}");
                println!("Movable range found: #{movable_range:?}");
                if let Some(movable_range) = movable_range {
                    new_leftover_ranges.append(&mut set_difference(*range, movable_range));
                    let diff = mapping.1 - mapping.0;
                    new_ranges.push((movable_range.0 - diff, movable_range.1 - diff));
                } else {
                    new_leftover_ranges.push(*range);
                }
            }
            leftover_ranges = new_leftover_ranges;
        }
        new_ranges.append(&mut leftover_ranges);
        new_ranges
    })
}

fn part_two() {
    let (seeds, areas) = parse_input("input.txt");

    println!("Part 2");

    let result = seeds
        .chunks(2)
        .flat_map(|range| traverse_interval_maps((range[0], range[0] + range[1]), &areas))
        .map(|x| x.0)
        .min()
        .unwrap();

    // println!("#{:?}", new_seeds);
    // println!("#{:?}", areas);
    println!("#{:?}", result);
}

fn main() {
    part_one();
    part_two();
}
