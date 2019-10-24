fn main(){
    let maybe_value = Some(2);
    match maybe_value {
        Some(value) if value == 2=> {
            println!("some is dos");
        }
        Some(value) => {
            println!("not None: {}", value);
        }
        None =>{
            println!("None");
        }
    }
}
