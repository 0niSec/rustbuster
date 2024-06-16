mod cli;
mod parser;

use cli::Args;


fn main() {
    let url = parser::parse_url();
    
    println!("{:?}", url);
}
