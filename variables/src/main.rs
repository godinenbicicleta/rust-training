fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let s1 = String::from("hello");
    let s2 = s1;  //s1 is moved into s2 instead of shallow copy

//  println!("{}, world", s1);  s1 no longer exists
    println!("{}, world", s2);
    // if we do want to deeply copy the the heap data of the
    // string, not just the stack data (pointer, len, capacity)
    // we can use clone

    let s3 = String::from("Hello");
    let s4 = s3.clone();

    println!("s1 = {}, s2 = {}", s3, s4);

    // but what happens with integers?
    let y = 1;
    let z = y;
    //both z and y remain valid
    println!("y = {}, z = {}", y,z);

    //ints are on the stack because of fixed
    //size and thus copying is fast
    //
    let s = String::from("hello");

    takes_ownership(s);
    //s is no longer valid
    //println!("{}",s);  does not compile, s no longer exists

    let x = 5;
    makes_copy(x);
    println!("x is {}",x);

    //passing references instead of using borrow and ownership
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of '{}' is {}.", s1, len);
}

fn makes_copy(x: i32) ->(){
    println!("some integer, {}",x);
}

fn takes_ownership(some_string: String){
    // some_string comes into scope
    println!("{}", some_string);
    // some_string goes out of scope and memory is freed
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
