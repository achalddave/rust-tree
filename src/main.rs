mod tree;

fn main() {
    match tree::tree() {
        Ok(_) => {},
        Err(_) => println!("There was an error!")
    }
}
