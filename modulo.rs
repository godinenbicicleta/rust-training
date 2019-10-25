mod modulo00{
    pub struct Dato(pubpub  u32, pub u32);

    pub trait Consumible {
        fn consumir(par01: &Dato) -> (u32,u32);
    }

}

use modulo00::Consumible;
use modulo00::Dato;
fn main(){

}

fn test_modulo00(){
    struct MiImpl{
        cadena: String,
    }

    impl Consumible for MiImpl{
        fn consumir(par01: &Dato) -> (u32, u32){
          (par01.0, par01.1)
        }
    
    }
}
