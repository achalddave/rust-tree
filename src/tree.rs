use std::fs;
use std::io;
use std::iter;

// Specify indent size?

pub fn tree(path: &str) -> Result<(), io::Error>{
    return print_dir_recursive(path, 0);
}

fn print_dir_recursive(path : &str, indent_level : usize)
        -> Result<(), io::Error>{
    let indent : String = iter::repeat("   |").take(indent_level).collect();

    let paths = try!(fs::read_dir(path));
    for entry in paths {
        // For some reason, try!(entry).path().to_str().unwrap() complains about
        // some lifetime issue? I don't understand this yet.
        let entry_name = try!(entry).path();
        let entry_name = entry_name.to_str().unwrap();
        println!("{}- {}", indent, entry_name);
        print_dir_recursive(entry_name, indent_level + 1);
    }
    return Ok(());
}
