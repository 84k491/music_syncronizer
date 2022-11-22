/// unified source pool

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
use crate::object::ActionType;
use std::collections::VecDeque;

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

fn pool_with_largest_space<'a>(pools: &'a mut Vec::<Pool>) -> &'a mut Pool {
    pools.get_mut(0).unwrap()
}

// TODO check size of the objects
fn main() -> Result<(), i32> {
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
                if index != args.len() - 1 {
                    origins.entry(v).or_default().push(args[index + 1].clone());
                }
            }
            None => {}
        }
    }

    let mut source_pool_united = Pool::new(OriginType::Source);
    match origins.get(&OriginType::Source) {
        Some(vec_of_paths) => {
            for path in vec_of_paths {
               source_pool_united.inflate_from_path(&path);
            }
        },
        _ => panic!("No source specified"),
    };

    let mut destination_pools = Vec::<Pool>::new();
    match origins.get(&OriginType::Destination) {
        Some(vec_of_paths) => {
            for path in vec_of_paths {
                let mut pool = Pool::new(OriginType::Destination);
                pool.inflate_from_path(path);
                destination_pools.push(pool);
            }
        },
        _ => panic!("No destination specified"),
    };

    for pool in &mut destination_pools {
        pool.remove_difference(&source_pool_united);
    }

    // TODO to guarantee, that there will be enough space after this point

    let mut copy_queue = 
        VecDeque::from(source_pool_united.extract_difference_with_multiple_pools(&destination_pools));
    // files will be moved out on the line above
    while let Some(obj) = copy_queue.pop_front() {
        let target_pool = pool_with_largest_space(&mut destination_pools);

        if !target_pool.has_available_space(obj.size) {
            copy_queue.append(&mut VecDeque::from(target_pool.extract_for_free_space(obj.size)));
        }

        target_pool.push(obj);
    }

    // TODO check if no errors, show results, ask to proceed

    for pool in &mut destination_pools {
        pool.invoke_actions_with_type(ActionType::Remove);
    }

    for pool in &mut destination_pools {
        pool.invoke_actions_with_type(ActionType::MoveIn);
        // TODO there will be miscalculated space it we won't remove move out actions

        // TODO here can be an error with "not enough space" when moving a lot to a pool,
        // which has some objects to move after
    }

    for pool in &mut destination_pools {
        pool.invoke_actions_with_type(ActionType::CopyIn);
    }

    Ok(())
}
