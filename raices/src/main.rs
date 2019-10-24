use std::env;
use std::fmt;

struct NumeroComplejo{
    real: f64,
    img: f64,
}
enum Raiz{
    PrimerGrado(f64),
    SegundoGradoReal(f64, f64),
    SegundoGradoComplejo(NumeroComplejo, NumeroComplejo),

}


impl fmt::Display for NumeroComplejo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.img>0.0{
            write!(f, "{} +{}i", self.real, self.img)
        } else{
            write!(f, "{} {}i", self.real, self.img)
        
        }
    }
}

impl fmt::Display for Raiz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Raiz::PrimerGrado(value) => write!(f, "x = {}", value),
            Raiz::SegundoGradoReal(value1, value2) => write!(f, "x1 = {}, x2 = {}", value1, value2),
            Raiz::SegundoGradoComplejo(r1, r2) => write!(f, "x1 = {}, x2 = {}", r1, r2),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = String::from("-a");
    let b = String::from("-b");
    let c = String::from("-c");
    let discriminante = String::from("-discriminante");
    let roc = String::from("-roc");

    match &args[..]{
        [_, alabel, a_value,blabel, b_value, clabel, c_value] 
            if a == *alabel && b == *blabel && c == *clabel
            => {
                let (a,b,c) = parse_values(a_value, b_value, c_value);
                raices(a,b,c)
            }
        [_, alabel, a_value,blabel, b_value, clabel, c_value, disc] 
            if a == *alabel && b == *blabel && c == *clabel && *disc == discriminante
            =>{
                let (a,b,c) = parse_values(a_value, b_value, c_value);
                println!("{}",get_discriminante(a,b,c))
            },
        [_, alabel, a_value,blabel, b_value, clabel, c_value, roc_value] 
            if a == *alabel && b == *blabel && c == *clabel && *roc_value == roc
            =>{
                let (a,b,c) = parse_values(a_value, b_value, c_value);
                get_roc(a,b,c)
            },
        _ => println!("Invalid Options"),
    }

}

fn raices(a: f64,b: f64,c: f64)->(){
    let discriminante = get_discriminante(a,b,c);
    if a == 0.0 {
        primer_orden(b,c)
    } else if discriminante >= 0.0 {
        raices_reales(a,b,discriminante)
    } else{ 
        raices_complejas(a,b,discriminante)
    }
}

fn get_discriminante(a: f64,b: f64,c: f64)-> f64{
    b*b-4.0*a*c
}

fn get_roc(a: f64,b: f64,c: f64)-> (){
    let discriminante = get_discriminante(a,b,c);

    if discriminante >= 0.0 {
        println!("reales")
    } else{
        println!("complejas")
    }
}

fn primer_orden(b: f64,c: f64) ->(){
    let resultado = Raiz::PrimerGrado(-1.0*c/b);
    println!("{}", resultado)
}

fn raices_complejas(a: f64, b: f64,discriminante: f64) ->(){
    //-b/2a+-sqrt(b2-4ac)/2a
    let real = -1.0*b/2.0/a;
    let img = (-1.0 *discriminante).sqrt()/2.0/a;
    let resultado = Raiz::SegundoGradoComplejo(
        NumeroComplejo{real:real, img:img}, 
        NumeroComplejo{real:real, img:-1.0*img}
        );
    println!("{}", resultado)

}

fn raices_reales(a: f64, b: f64,discriminante: f64) ->(){
    let r1 = -1.0*b/2.0/a;
    let r2 = discriminante.sqrt()/2.0/a;
    let resultado = Raiz::SegundoGradoReal(r1+r2,r1-r2);
    println!("{}", resultado)
}

fn parse_values(a: &String,b: &String,c: &String)->(f64, f64, f64){
    (
        a.parse().expect("invalid a"),
        b.parse().expect("invalid b"),
        c.parse().expect("invalid c")
        )
}
