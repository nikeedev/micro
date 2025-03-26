use std::{
    env,
    fs,
    process::exit,
    collections::HashMap
};

fn get_value_arg<'a>(from: &'static str, v: &'a[String]) -> Option<&'a String> {
    let index = v.iter().position(|r| r == from).unwrap();

    return v.get(index + 1_usize);
    // println!("Value for {:?} is not defined!", from);
    // exit(1);
}

fn compress(text: String) -> String {
    println!("[1/3] Compressing..."); 
    
    let mut chars: Vec<char> = text.chars().collect();
    
    let mut count: HashMap<char, u128> = HashMap::new();

    chars.iter().for_each(|x| {
        if count.contains_key(&x) {
            *count.get_mut(&x).unwrap() += 1 as u128;
        } else {
            count.insert(*x, 1);
        }
    });
    
    let mut var_count: u32 = 0b1;
    count.iter().for_each(|x| {
        if *x.1 > 1 {
            chars.iter_mut().for_each(|c| {
                if *c == *x.0 {
                    *c = std::char::from_u32(0b0100 + var_count).unwrap_or('?');
                }
            });
            chars.insert(0, std::char::from_u32(0b0100 + var_count).unwrap_or('?'));
            chars.insert(1, *x.0);
            var_count += 0b001;
        }
    });
    
    let new_text: String = chars.into_iter().collect();
        
    println!("[2/3] Done compressing!");
    new_text
}

fn decompress(text: String) {
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    
    if args.contains(&"help".to_string()) || args.len() < 2 {
        println!(":: micro :: CV1 compressor and decompressor - v0.1.6");
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
        
        let new_file = match fs::read_to_string(temp) {
            Ok(x) => compress(x),
            Err(x) => panic!("Error reading \"{}\": {}", temp, x),
        };
        
        let mut temp: String = temp.to_owned(); 
        temp.push_str(".cv1");
        println!("[3/3] Saving to file: {}", temp); 
        fs::write(temp, new_file).unwrap(); 
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
            Ok(x) => decompress(x),
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
