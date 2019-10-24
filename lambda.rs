fn main(){
  let value = Some(1).map(|v| v+1);
  println!("{}", value.unwrap());

  let value = Some(1).map(|v| {
    v + 1
  });

  let closure = |v| v+1;

  let value = Some(1).map(closure);
}
