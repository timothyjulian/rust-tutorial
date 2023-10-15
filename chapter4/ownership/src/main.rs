fn main() {
    let x: String = String::from("Test");
    gives_ownership(x); // giving the value to function (move), therefore x is no longer valid
    // println!("{}", x); // the ownership is already given to the function therefore this line will error

    let asd: i32 = 5;
    just_copy(asd); 

    println!("{}", asd); // this will not error because the function copied the value not taking
                         // ownership


    let heap: String = String::from("heap");
    let new_heap: String = heap; // need to do heap.clone() to make a copy

    // println!("{} {}", heap, new_heap); // will result error because heap is moved and not valid
    let stack: i32 = 5;
    let new_stack: i32 = stack;

    println!("{} {}", stack, new_stack); // will be fine because the value is not copied (it is stored in stack )

    let mut s1 = String::from("hello");

    let length = borrowing(&s1); // s1 is borrowed by using reference

    borrow_and_mutate(&mut s1);

    println!("{s1}"); // because s1 is not moved this still valids

    let hheh: &String = &s1;
 
    println!("{hheh}");

    let hehe2: &mut String= &mut s1;
    borrow_and_mutate(hehe2);

    println!("hehe2: {hehe2}");
    println!("from s1: {s1}")
} 

// fn dangle() -> &String { // this will error because the s will be out of scope but the reference is
//                          // given back to the caller of the function
//     let s: String = String::from("test");
// 
//     &s
// }
//

fn no_dangle() -> String {
    let s: String = String::from("asd");

    s // will return the ownership so will not be a problem because s i not allocated as the
      // function will no more be the owner
}

fn just_copy(test: i32){
    println!("{}", test);
}

fn gives_ownership(a_string: String){
    println!("{}", a_string);
} // a_string is out of scope and memory is release by drop

fn borrowing(a_string: &String) -> usize{ // create a pointer a_string that points to s1
    a_string.len()
} // a_string is out of scope but the string is not drop

fn borrow_and_mutate(a_string: &mut String) { // create a mutable reference
    a_string.push_str("test");
}
