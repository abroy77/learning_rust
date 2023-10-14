use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");

    // let nums = vec![2, 7, 11, 15];
    // let target = 9;

    // let result = two_sum_hash_onepass(nums, target);
    // println!("{:?}", result);

    let mut x = Box::new(5);
    *x += 5;

    println!("{}", x);
}

pub fn two_sum_ref(nums: &Vec<i32>, target: i32) -> Option<(i32, i32)> {
    // function that returns indices of the two elements of a vector
    // such that they sum to a target

    // loop through the vector
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if i != j && x + y == target {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

fn two_sum_consume(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if i != j && x + y == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    //make a hashmap of element: index

    for (i, x) in nums.iter().enumerate() {
        hashmap.insert(*x, i as i32);
    }
    // loop through and get complement
    for (i, x) in nums.iter().enumerate() {
        let complement = target - x;
        if hashmap.contains_key(&complement) && hashmap[&complement] != i as i32 {
            return vec![i as i32, hashmap[&complement]];
        }
    }
    unreachable!("No solution found")
}

fn two_sum_hash_onepass(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for (i, x) in nums.iter().enumerate() {
        let complement: i32 = target - x;
        match hashmap.get(&complement) {
            Some(&j) => return vec![i as i32, j],
            None => {
                hashmap.insert(*x, i as i32);
            }
        }
    }
    unreachable!("No solution found")
}
