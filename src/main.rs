use std::fs::*;
extern crate json;
use rand::prelude::Rng;
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let fstr = read_to_string("data/hayfacts.json")?; 
    let parsed = json::parse(&fstr)?;
    let facts_num = &parsed["facts"].len();
    let mut rng = rand::thread_rng();
    let rand: usize = rng.gen_range(0,facts_num); 
    println!("{}", parsed["facts"][rand]);
    Ok(())
}