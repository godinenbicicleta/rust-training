fn main(){
    let arreglo: [u32;16] = [0;16];
    let mut arreglo_dinamico: Vec<u32> = Vec::new();

    println!("{:?}", arreglo_dinamico);

    for i in 0..105{
      arreglo_dinamico.push(i);
    }
    
    println!("{:?}", arreglo_dinamico);
}
