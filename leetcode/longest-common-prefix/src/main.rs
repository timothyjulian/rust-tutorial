use std::{f32::MIN, i32, ptr::null};

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut shortest_length = i32::MAX;
        let mut shortest_length_index: i32 = 0;
        for index in 0..strs.len() {
            shortest_length = if (strs.get(index).unwrap().len() as i32) < shortest_length {
                shortest_length_index = index as i32;
                strs.get(index).unwrap().len() as i32
            } else {
                shortest_length
            }
        }
        let mut longest_index: i32 = i32::MIN;
        for index in 0..shortest_length {
            let mut is_common = true;
            let mut last_char: char = '0';
            for word in 0..strs.len() {
                let char = strs.get(word).unwrap().chars().nth(index as usize).unwrap();
                // println!("{} {} {}",strs.get(word).unwrap(), char, last_char);
                if last_char == '0' {
                    last_char = char;
                    continue;
                }

                if char != last_char {
                    is_common = false;
                    break;
                }
            }
            last_char = '0';
            if !is_common {
                longest_index = index - 1;
                break;
            }
        }
        // println!("{}", longest_index);

        let shortest_word = strs.get(shortest_length_index as usize).unwrap();
        if longest_index == i32::MIN {
            // println!("{}", shortest_word);
            return shortest_word.to_string();
        } else if longest_index == -1 {
            return String::new();
        } else {
            return (&shortest_word[0..=longest_index as usize]).to_string();
            // println!("{}", &shortest_word[0..=longest_index as usize]);
        }


    }
}

#[test]
pub fn test_longest_common_prefix() {
    assert_eq!(
        Solution::longest_common_prefix(
            [
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]
            .to_vec()
        ),
        String::from("fl")
    );


    assert_eq!(
        Solution::longest_common_prefix(
            [
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]
            .to_vec()
        ),
        String::from("")
    );
}

fn main() {
    println!("Hello, world!");
}
