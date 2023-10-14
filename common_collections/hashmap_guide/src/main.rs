use rand::Rng;
use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score is {}", score);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    //OVERWRITING

    println!("{:?}", scores);

    // get median of an unsorted vector

    // make random vector

    let mut gen = rand::thread_rng();
    let v: Vec<i32> = (0..10).map(|_| gen.gen_range(0..100)).collect();

    println!("input vector {:?}", v);
    let median = get_median(&v);

    println!("median is {}", median);

    let avg = get_average(&v);

    println!("average is {}", avg);

    let mode = get_mode(&v);
    println!("mode is {mode}");
}

fn get_median(v: &[i32]) -> i32 {
    let mut v = v.to_vec();
    v.sort();
    let length = v.len();
    if length % 2 == 0 {
        let mid = length / 2;
        return (v[mid] + v[mid - 1]) / 2;
    }
    let mid = v.len() / 2;
    v[mid]
}

fn get_average(x: &[i32]) -> i32 {
    x.iter().sum::<i32>() / x.len() as i32
}

fn get_mode(v: &[i32]) -> i32 {
    let mut counter = HashMap::new();
    for element in v {
        let count = counter.entry(element).or_insert(0);
        *count += 1;
    }
    let mode = counter
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap_or((&0, 0));

    *mode.0
}
