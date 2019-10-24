//function declaration
fn add_them(first: i32, second: i32) -> i32{
  first + second
}

fn main(){
  //mutable variable
  let mut some_value = 1;

  //immutable, explicit type
  let explicitly_typed: i32 = 1;

  //function call
  some_value = add_them(some_value, explicitly_typed);

  //macro
  println!("{}", some_value);
}
