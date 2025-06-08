// notes:
// 0x000001 is for setting variable + variable code
// 0x000002 is for getting variable + variable code


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
    println!("[1/2] Compressing..."); 
    
    let chars: Vec<char> = text.chars().collect();
    
    let mut count: HashMap<char, u128> = HashMap::new();

    chars.iter().for_each(|x| {
        if count.contains_key(&x) {
            *count.get_mut(&x).unwrap() += 1 as u128;
        } else {
            count.insert(*x, 1 as u128);
        }
    });
    
    let mut new_chars: Vec<char> = Vec::new();

    let mut var_count: u32 = 0x000001;
    
    let mut vars: HashMap<char, char> = HashMap::new();

    count.iter().for_each(|x| {
        if *x.1 > 1 {
            // add variable assign signal
            new_chars.push(std::char::from_u32(0x000001).unwrap());
            // variable name: 
            new_chars.push(std::char::from_u32(var_count).unwrap());
            // variables value
            new_chars.push(std::char::from_u32(*x.0 as u32+0x0000ff).unwrap());
            
            vars.insert(*x.0, std::char::from_u32(var_count).unwrap());

            var_count += 0x000001;
        }
    });

    // println!("new vars: {:#?}", vars);

    chars.iter().for_each(|x| {
        if vars.contains_key(&x) {
            new_chars.push(std::char::from_u32(0x000002).unwrap());
            new_chars.push(vars[&x]);
        } else {
            new_chars.push(std::char::from_u32(*x as u32 + 0x0000ff).unwrap());
        }
    });

    // println!("new char: {:#?}", new_chars);

    let new_text: String = new_chars.into_iter().collect();
        
    println!("[1/2] Done compressing!");
    new_text
}

fn decompress(text: String) -> String {
    println!("[1/2] Decompressing..."); 

    let chars: Vec<char> = text.chars().collect();
    
    let mut vars: HashMap<u32, char> = HashMap::new();
    
    let mut i = 0;
    
    let mut new_chars: Vec<char> = Vec::new();

    while i < chars.len() {
        if chars[i] as u32 == 0x000001 {
            if i + 2 >= chars.len() {
                break;
            }

            let key = chars[i + 1] as u32;
            let value = std::char::from_u32(chars[i + 2] as u32 - 0x0000ff).unwrap();

            vars.insert(key, value);
            
            // println!("var {}: {}", i, value);
            
            i += 3;
            continue;
        } 
        else if chars[i] as u32 == 0x000002 {
            let replace = vars[&(chars[i+1] as u32)];
            
            new_chars.push(replace);

            i += 2;
            continue;
        }
        else {
            
            let decoded = std::char::from_u32(chars[i] as u32 - 0x0000ff).unwrap();
            new_chars.push(decoded);
            // println!("norm char {}: {}", i, decoded);
        }   
        
        i += 1;
    }
    
    // println!("{:#?}", new_chars);

    println!("[1/2] Done decompressing!");

    let output: String = new_chars.into_iter().collect();
    
    output
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
        println!("[2/2] Saving to file: {}", temp);
        fs::write(temp, new_file).unwrap(); 
        println!("[2/2] Done!");
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

        let output = match fs::read_to_string(temp) {
            Ok(x) => decompress(x),
            Err(x) => panic!("Error reading \"{}\": {}", temp, x),
        };

        let trimmed = match temp.rfind('.') {
            Some(index) => &temp[..index],
            None => temp,
        };
        
        println!("[2/2] Saving to {}", trimmed);

        let _ = fs::write(trimmed, output.as_str());

        println!("[2/2] Done!");
        
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
