trait Bar{
    fn default_implementation(&self) -> bool{
        true
    }
    fn required_implementation(&self);
}

impl Bar for Foo {
    fn required_implementation(&self){
        println!("this was required");
    }
}

impl Foo{
  fn new() -> Self{ Foo }
}
