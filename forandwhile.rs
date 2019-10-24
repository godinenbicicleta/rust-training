fn main(){
    let range = 0..10;
    for i in range{
        println!("{}",i);
    }

    let mut range = 0..10;
    while let Some(v) = range.next(){
        println!("{}",v);
    
    }
}
