struct Estructura<'a>{ // 'a es el ciclo de vida porque
                      // estamos usando una referencia
    campo01: u8,
    campo02: f32,
    campo03: &'a str,
}


fn main() {
}

#[test]
fn test_estructura(){
    let mut  una_cadena = "una cadena";
    let mut estructura = Estructura{ 
        campo01: 0x70,
        campo02: 3.1416,
        campo03: una_cadena,
    };
    estructura.campo03 = una_cadena;
    una_cadena = "otra cadena";

    assert!("una cadena" == estructura.campo03);
}
