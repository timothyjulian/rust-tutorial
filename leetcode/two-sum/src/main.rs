use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut answer_vec: Vec<i32> = Vec::new();
    let mut difference_map: HashMap<i32, i32> = HashMap::new();
    let mut i = 0;
    for num in nums {
        let difference = target - num;
        // println!("{} {} {:?}", num, difference, difference_map);
        let index_value = *difference_map.get(&num).unwrap_or(&-1); 
        if index_value == -1 {
            difference_map.insert(difference, i);
        } else {
            answer_vec.push(index_value);
            answer_vec.push(i);
        }
        i += 1;
    }

    answer_vec
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}

fn main() {
    println!("Hello, world!");
}
