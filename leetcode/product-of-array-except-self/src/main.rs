pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut prefix_product = Vec::new();
    let mut current = 1;
    for n in &nums{
        current *= *n;
        prefix_product.push(current);
    } 
    // println!("{:?}", prefix_product);

    nums.reverse();

    let mut suffix_product = Vec::new();
    let mut current = 1;
    for n in &nums {    
        current *= *n;
        suffix_product.push(current);
    }

    suffix_product.reverse();
    // println!("{:?}", suffix_product);

    let mut answer_vec = Vec::new();
    for i in 0..nums.len() {
        if i == 0 {
            answer_vec.push(*suffix_product.get(i+1).unwrap());
        } else if i == nums.len() - 1 {
            answer_vec.push(*prefix_product.get(i-1).unwrap());
        } else {
            let product_except_self = prefix_product.get(i-1).unwrap() * suffix_product.get(i+1).unwrap(); 
            answer_vec.push(product_except_self);
        }
    }

    // println!("{:?}", answer_vec);
        

    answer_vec
}

#[test]
fn test_product_except_self() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}

fn main() {
    println!("Hello, world!");
}
