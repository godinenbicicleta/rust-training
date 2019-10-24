fn main(){
  let var00 : u8 = 0;
  let var01 : i16 = -23450;
  let var02 : u32 = 1234455;
  let var03 : i64 = 123445598765;

  println!("{:p} {:p} {:p} {:p}",&var00, &var01, &var02, &var03 );

  let arreglo1 : [u8;8] = [0;8];
  let arreglo2 : [u8;16] = [0;16];
  println!("{:p} {:p}", &arreglo1, &arreglo2);

  struct GusGusGus{
    campo01: u8,
    campo02: u8
  }

  let gus01 = GusGusGus {campo01: 14, campo02:42};
  let gus02 = GusGusGus {campo01: 14, campo02:42};
  println!("{:p} {:p}", &gus01, &gus02);

  let heap00 = Box::new(16 as u8);
  let heap01 = Box::new(14 as u8);

  println!("{:p} {:p}", heap00, &heap00);
}
