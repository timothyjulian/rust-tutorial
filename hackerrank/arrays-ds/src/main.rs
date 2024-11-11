use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'reverseArray' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn reverse_array(a: &[i32]) -> Vec<i32> {
    let mut answer_vec: Vec<i32> = Vec::new();
    let mut final_index: isize = (a.len() - 1) as isize;
    
    while final_index >= 0 {
        answer_vec.push(a[final_index as usize]);
        final_index -= 1;
    }
    // a.iter().rev().cloned().collect();
    return answer_vec;
}

#[test]
fn test_reverse_array(){
    assert_eq!(reverse_array(&[5, 4, 3, 2, 1]), [1, 2, 3, 4, 5].to_vec());
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let res = reverse_array(&arr);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
