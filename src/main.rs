use std::env;
mod tree;

fn usage() {
    println!("Usage: tree [path]");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        return usage();
    }

    let mut path = ".";
    if args.len() > 1 {
        path = &args[1];
    }

    match tree::tree(path) {
        Ok(_) => {},
        Err(_) => println!("There was an error!")
    }
}
