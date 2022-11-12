// parse arguments. Multiple sources, Multiple destinations
/// unified source pool
/// class, that will represent a folder:
    /// path
    /// name
    /// origin (source / dest / tmp)

/// find every file in destinations with source. mark to remove absent

/// get total space needed in sources
/// compare with total space needed in destinations. minus removed

/// make a queue for copying folders from source to dest find the biggest available space for each

/// if for some folder there is no enough space in one dest, but it's ok in total,
/// then move some folder from dest to the queue
/// choose a folder with the closest but strictly smaller size

mod origin;
mod object;

use std::env;
use std::collections::HashMap;
use origin::OriginType;
use origin::Pool;

fn print_help() {
    println!("Tool for syncronizing folders in several destinations with multiple sources");
    println!("Usage: -s <source1> -s <source2> ... -d <destination1> -d <destination2> ...");
}

fn to_flag(s: &String) -> Option<OriginType> {
    let source_flag = "-s";
    let destination_flag = "-d";

    if source_flag == s {
        Some(OriginType::Source)
    }
    else if destination_flag == s {
        Some(OriginType::Destination)
    }
    else {
        None
    }
}

fn main() -> Result<(), i32>{
    println!("Start");

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_help();
        return Err(-1);
    }

    let mut origins: HashMap<OriginType, Vec<String>> = HashMap::new();

    for (index, arg) in args.iter().enumerate() {
        let flag = to_flag(&arg);
        match flag {
        Some(v) => {
            println!("There is a flag {:?}", v);
            if index != args.len() - 1 {
                origins.entry(v).or_default().push(args[index + 1].clone());
            }
        }
        None => { println!("No flag: {}", arg)}
        }
    }

    println!("Origins size: {}", origins.len());
    println!("");
    for (origin_type, path_vec) in origins {
        for path in path_vec {
            println!("Entry: {:?}: {}", origin_type, path);
            let pool = Pool::new(&origin_type, &path);
        }
    }
    Ok(())
}
