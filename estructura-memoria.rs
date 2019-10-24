#[derive(Debug)]
 struct GusGusGus {
       campo01: u8,
       campo02: u8,
  }
fn f001( par01: [u8;16]) -> () {
    println!("{:?} {:p}",par01, &par01)
}

fn f002(mut par01: &mut GusGusGus )-> () {
     par01.campo01 = par01.campo01 + par01.campo02;
     println!("{:?} {:p}",par01, &par01)
}
fn main() {
//muestra como se asigna memoria en el stack
   let var00 : u8 = 0;
   let var01 : i16 = -23450;
   let var02 : u32 = 134323434;
   let var03 : i64 = 23343534343343;
   let var04 : isize = 2334353434999;
   println!("{:p} {:p} {:p} {:p} {:p}", &var00, &var01, &var02,&var03,&var04);
   let var05 : f32 = 3.14159;
   let var06 : f64 = 3.14159;
   println!("{:p} {:p} ", &var05, &var06);
   //y los arreglos estaticos son parte del stack
   let arreglo: [u8;8] = [0;8];
   let arreglo2: [u8;16] = [0;16];
   println!("{:p} {:p} ", &arreglo, &arreglo2);
   //tambien las estructuras son parte del stack
   let gus01 = GusGusGus { campo01: 14, campo02: 42 };
   let mut gus02 = GusGusGus { campo01: 14, campo02: 42 };
    println!("{:p} {:p} ", &gus01,&gus02); 
    println!("{:p}", &main);
    //pero tambien existe el heap, usando Box::new
    let heap01 = Box::new(16 as u8);
    println!("{} {:p} {:p}", heap01, heap01, &heap01);
    let heap02 = Box::new( [0;16] ) ;
    println!("{:?} {:p} {:p}", heap02, heap02, &heap02);
    //y el arreglo estatico se puede enviar a funciones
    f001(arreglo2);
    //y la estructura estatica se puede prestar a una funcion
    f002(&mut gus02);
    println!("{:?} {:p} ", gus02,&gus02);
    let mut gus03 = Box::new(GusGusGus { campo01: 128, campo02: 25 });
    f002(&mut gus03);
     println!("{:?} {:p} ", gus03,&gus03);
}
