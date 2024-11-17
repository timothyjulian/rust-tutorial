use std::{collections::HashMap, i32};

// pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
//     let mut nums_mut = nums;
//     nums_mut.sort();

//     let mut i: i32 = 0;
//     let mut count: i32 = 0;
//     let mut last_num: i32 = i32::MIN;
//     let mut ordered_map: Vec<(i32, i32)> = Vec::new();
//     for num in &nums_mut {
//         if i == 0 {
//             last_num = *num;
//             count += 1;
//             i += 1;
//             continue;
//         }
//         if *num == last_num {
//             count += 1;
//         } else {
//             // println!("{num} {last_num} {count}");
//             ordered_map.push((count, last_num));
//             count = 1;
//             last_num = *num;
//             // println!("{:?}", ordered_map);
//         }
//         i += 1;
//     }


//     // consider last element
//     ordered_map.push((count, last_num));
//     ordered_map.sort_by(|a, b| b.0.cmp(&a.0));

//             // println!("{:?}", ordered_map);

//     // let mut j = 0;
//     let mut answer_vec = Vec::new();
//     for j in 0..k {
//         answer_vec.push(ordered_map.get(j as usize).unwrap().1);
//     }
//     // for (_, v) in ordered_map.into_iter().rev() {
//     //     if j >= k {
//     //         break;
//     //     }
//     //     answer_vec.push(v);
//     //     println!("{k} {v}");
//     //     j += 1;
//     // }

//     answer_vec
// }

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut hash_map: HashMap<i32, i32> = HashMap::new();
    for i in nums {

        if let Some(value) = hash_map.get_mut(&i) { 
            *value += 1;
        } else {
            hash_map.insert(i, 1);
        }

    }
    let mut hash_vec: Vec<(i32, i32)> = hash_map.into_iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(&a.1));
    for i in 0..k {
        result.push(hash_vec[i as usize].0);
    }

    result
}

#[test]
fn test_top_k_frequent() {
    println!("{:?}",top_k_frequent(vec![1,1,1,2,2,3], 2));
    println!("{:?}",top_k_frequent(vec![1], 1));
    println!("{:?}",top_k_frequent(vec![1,2], 2));

}

fn main() {
    println!("Hello, world!");
}
