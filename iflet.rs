fn main(){
  let maybe_value = Some(2);
  if let Some(5) = maybe_value {
      println!("value is maybe_value");
  } else {
      println!("value is not some value");
  }
}
