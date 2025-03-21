use std::{env, fs, process::{self, exit}};


fn get_value_arg<'a>(from: &'static str, v: &'a[String]) -> Option<&'a String> {
    let index = v.iter().position(|r| r == from).unwrap();

    return v.get(index + 1_usize);
    // println!("Value for {:?} is not defined!", from);
    // exit(1);
}

fn compress(text: String) {
    
}

fn decompress(text: String) {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    
    if args.contains(&"help".to_string()) || args.len() < 2 {
        println!("usage: micro [command] <file>");
        println!("command:");
        println!("\thelp: shows this message");
        println!("\tcomp: compresses a file, pass the file that will be compressed");
        println!("\tdecomp: decompresses a file, pass the file that will be decompressed,\n\t\tmust be .cv1 file extension (this is to prevent the program from misbehaving if a different file is passed and the algorithm fails)");

        exit(0);
    }

    if args.contains(&"comp".to_string()) {
        let temp = get_value_arg("comp", &args);

        if temp.is_none() {
            eprintln!("Value for \"comp\" was not defined!");
            exit(0);
        }
        
        let temp = temp.unwrap();
        match fs::read_to_string(temp) {
            Ok(x) => compress(x),
            Err(x) => panic!("Error reading \"{}\": {}", temp, x),
        } 
    }
   
    if args.contains(&"decomp".to_string()) {
        let temp = get_value_arg("decomp", &args);

        if temp.is_none() {
            eprintln!("Value for \"decomp\" was not defined!");
            exit(0);
        }
        
        let temp = temp.unwrap();

        if !temp.ends_with(".cv1") {
            eprintln!("Invalid file extension. File extension must be .cv1 (this is done to prevent program from misbehaving if a different file is passed)");
            exit(0x1)
        }

        match fs::read_to_string(temp) {
            Ok(x) => compress(x),
            Err(x) => panic!("Error reading \"{}\": {}", temp, x),
        } 
    }


    /*
    if !args[1].ends_with(".cv1") {
        eprintln!("Invalid file extension. File extension must be .cv1 (this is done to prevent program from misbehaving if a different file is passed)");
        exit(0x1)
    }

    match fs::read_to_string(args[1].clone()) {
        Ok(x) => x,
        Err(x) => panic!("Error reading \"{}\": {}", args[1], x),
    } 
    */
}
