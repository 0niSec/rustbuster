use rustbuster::cli::{self, Args};

fn main() {
    let args = <Args as clap::Parser>::parse(); // Not sure this is the right way to do this
    
    println!("{:?}", args);
}
