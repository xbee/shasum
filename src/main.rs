extern crate octavo;
extern crate clap;
use octavo::octavo_digest::prelude::*;
use clap::{Arg, App};
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;


pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    strs.join("")
}


fn calculate_sha512(path: &Path) -> String {
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut s = Vec::new();

    match file.read_to_end(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => {
            let data = s;
            let mut result = vec![0; sha2::Sha512::output_bytes()];
            let mut sha = sha2::Sha512::default();

            sha.update(data);
            sha.result(&mut result);
            String::from(to_hex_string(result))
        }
    }
}


fn calculate_sha256(s: &Path) -> String {
    let path = Path::new(&s);
    let display = path.display();
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("Couldn't read {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = Vec::new();

    match file.read_to_end(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => {
            let data = s;
            let mut result = vec![0; sha2::Sha256::output_bytes()];
            let mut sha = sha2::Sha256::default();

            sha.update(data);
            sha.result(&mut result);
            String::from(to_hex_string(result))
        }
    }
}


fn calculate_sha1(path: &Path) -> String {

    let mut results = Vec::new();
    match path.is_dir() {
        true => {

            let paths = fs::read_dir(path).unwrap();
            for entry in paths {
                if let Ok(entry) = entry {
                    // Here, `entry` is a `DirEntry`.

                    let mut file = match fs::File::open(entry.path()) {
                        Err(why) => {
                            panic!("Couldn't read {}: {}", path.display(), why.description())
                        }
                        Ok(file) => file,
                    };

                    let mut s = Vec::new();
                    if entry.path().is_file() {
                        match file.read_to_end(&mut s) {
                            Err(why) => {
                                panic!("couldn't read {}: {}, {:?}",
                                       path.display(),
                                       why.description(),
                                       entry.path())
                            }
                            Ok(_) => {
                                let data = s;
                                let mut result = vec![0; Sha1::output_bytes()];
                                let mut sha = Sha1::default();

                                sha.update(data);
                                sha.result(&mut result);
                                results.push(to_hex_string(result) + "   " +
                                             entry.path().to_str().unwrap());
                            }
                        }
                    }
                }
            }
            results.join("\n")

        }

        false => {
            let mut file = match fs::File::open(&path) {
                Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
                Ok(file) => file,
            };

            let mut s = Vec::new();

            match file.read_to_end(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
                Ok(_) => {
                    let data = s;
                    let mut result = vec![0; Sha1::output_bytes()];
                    let mut sha = Sha1::default();

                    sha.update(data);
                    sha.result(&mut result);
                    String::from(to_hex_string(result) + "    " + path.to_str().unwrap())
                }
            }
        }
    }
}


fn main() {
    let matches = App::new("shasum in rust")
        .version("1.0")
        .author("Smirnov V. <smirnovvad7@gmail.com>")
        .about("Calculate sha hashes for files")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("algorithm")
                 .short("a")
                 .takes_value(true)
                 .possible_values(&["1", "256", "512"])
                 .default_value("1"))
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let path = Path::new((matches.value_of("INPUT").unwrap()));
    match matches.value_of("algorithm").unwrap() {
        "1" => println!("{}", calculate_sha1(&path)),
        "256" => println!("{}   {}", calculate_sha256(&path), &path.display()),
        "512" => println!("{}   {}", calculate_sha512(&path), &path.display()),
        _ => unreachable!(),
    }
}