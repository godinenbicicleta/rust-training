use std::io;

fn main(){
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    println!("ax2 + bx + c = 0");
    
    println!("a = ");
    io::stdin().read_line(&mut a)
        .expect("Failed to read");
    
    println!("b = ");
    io::stdin().read_line(&mut b)
        .expect("Failed to read");
    
    println!("c = ");
    io::stdin().read_line(&mut c)
        .expect("Failed to read");

    let a1: f64 = a.trim().parse().unwrap();
    let b1: f64 = b.trim().parse().unwrap();
    let c1: f64 = c.trim().parse().unwrap();

    //-b +- sqrt(b2-4ac)
    if b1*b1 >= 4.0*a1*c1 {
    
        let sqrt_root: f64 = (b1*b1-4.0*a1*c1).sqrt();
        let root1: f64 = (-1.0*b1 + sqrt_root)/(2.0*a1);
        let root2: f64 = (-1.0*b1 - sqrt_root)/(2.0*a1);
        
        println!("x1 =  {:.2}, x2 = {:.2}", root1, root2);
    }
    else {
        println!("no real solutions");
    }
}
