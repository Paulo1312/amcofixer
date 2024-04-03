use amcofixer_lib::fixer;
use std::fs::File;
use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        let file_name = args[1].clone();
        match File::open(&file_name){ //Скучные проверки
            Ok(mut file1) => {
                let mut data1 = String::new();
                match file1.read_to_string(&mut data1) { //Скучные проверки
                    Ok(_) => { 
                        
                        // Я может выведу распаковку / Запаковку qcompress отдельно, но, пока что, пусть так будет.
                    
                        let encoded_data1 = fixer(data1);
                        
                        match File::create(file_name.replace(".vpn", ".fixed.vpn")) { //Скучные проверки
                            Ok(mut file2) => {
                                match file2.write_all(format!("vpn://{}\n", &encoded_data1).as_bytes()) {
                                    Ok(_) => (),
                                    Err(err) => print!("error: {}", err)
                                };
                            }
                            Err(err) => println!("Err: {}", err)
                        }
                    }
                    Err(err) => print!("{}", err)
                }
            }
            Err(err) => println!("{}", err)
        }
    }
    else{
        println!("Error(")
    }
}
