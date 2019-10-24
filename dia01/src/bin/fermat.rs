fn fermat(a: u64, b: u64) -> u64{
    a*a*a + b*b*b
}

fn main(){
    println!("fermat(3,4) = {}", fermat(3,4) )
}

#[test]

fn fermat_works(){
    assert!(fermat(3,4) == 27 + 64);
}
