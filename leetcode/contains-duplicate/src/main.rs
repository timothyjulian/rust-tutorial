use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut stored_num = HashMap::new();
    for num in nums {
        let value: i32 = *stored_num.get(&num).unwrap_or(&0);
        if value == 0 {
            stored_num.insert(num, 1);
        } else {
            return true;
        }
    }
        
    return false;
}

#[test]
fn test_contains_duplucate() {
    assert_eq!(contains_duplicate([1,2,3,1].to_vec()), true);
    assert_eq!(contains_duplicate([1,2,3,4].to_vec()), false);
    assert_eq!(contains_duplicate([1,1,1,3,3,4,3,2,4,2].to_vec()), true);
}


fn main() {
    println!("Hello, world!");
}
