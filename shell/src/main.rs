//Escribir un shell minimalista usando Rust.
//Dicho shell debe:
//Tomar de entrada estándar el comando a ejecutar del sistema opeativo
//Ejectuar el comando del sistema operativo
//Mostrar el resultado
//
//El tipo de comando a ejecutar es:
//* Un comando basico (ls)
//* Comando con opciones (ls -l por ejemplo)
//* Comando separado por pipes (ls -l | wc -l)
//* Arracnar un comando enviando a background  ( sleep 30000 &)
//
use std::process::Command;
use std::env;
use std::io::{self, Write};
extern crate regex;
use regex::Regex;

//fn parse_cmd(args: &String) -> Option<regex::Captures>{
//    let re_cmd = Regex::new(r"\s(\w+)").unwrap();
//    re_cmd.captures(args)
//}


fn parse_opts(args: &String) -> Vec<String>{
    let re_opts = Regex::new(
        r"[\w]?+\s(-?\w+\.?\w?+)"
        ).unwrap();
    re_opts.captures_iter(args)
       .map(|x| {
           //println!("{:?}",x);
           String::from(x.get(1).unwrap().as_str())
       })
       .collect()
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let args = args.join(" ");
    let piped = args.split("|").map(|z| String::from(z));
    for piped_args in piped{
        let opts = parse_opts(&piped_args);
        if opts.is_empty(){
            std::process::exit(1);
        }
        let output = run_command(&opts[0],
                                 &opts[1..].to_vec());
        io::stdout().write_all(&output.stdout).unwrap();
    }
}
    
    

fn run_command(cmd: &String,arguments: &Vec<String>) -> std::process::Output {
    //println!("{:?}",arguments);
    Command::new(cmd)
        .args(arguments)
        .output()
        .expect("Invalid command")
}

#[test]
fn test_ls(){
    let output = ls();
    assert_eq!(String::from_utf8_lossy(&output.stdout),
    "Cargo.lock\nCargo.toml\nsrc\ntarget\n")
}
