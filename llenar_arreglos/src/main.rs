fn llenar01(dato: u8) -> [u8;128]{
    let mut arreglo: [u8;128] = [0;128];

    for elemento in 0..128{
        arreglo[elemento] = dato;
    }
    arreglo
}

fn llenar02(buffer: &mut [u8;128], dato: u8) ->(){
    for i in 0..128{
        buffer[i] = dato;
    }

}


fn main(){
}

#[test]
fn tstllenar(){
    let arreglo = llenar01(0x40);
    assert!(arreglo[1] == 0x40);
    assert!(arreglo[127] == 0x40);
    assert!(arreglo.len() == 128);

    let mut arreglo02: [u8;128] = [0;128];

    llenar02(&mut arreglo02, 0x50);

    assert!(arreglo02[0] == 0x50 );
}
