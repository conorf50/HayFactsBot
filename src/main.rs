use std::fs::File;
fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let file = File::open("data/hayfacts.csv")?;
    let mut vals: Vec<csv::StringRecord> = Vec::new();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        //println!("{:?}", record);
        vals.push(record);
    }
    
    println!("Today's cool hay fact: {:?}", vals.to_vec()[0].as_slice());

    Ok(())
}