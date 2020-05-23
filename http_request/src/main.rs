extern crate reqwest;

use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
 
    // Simple GET
    let mut response = reqwest::get("http://localhost:8000")?;
    println!("{:#?}", response);
    println!("{:#?}", response.text()?);
    

    // Posting JSON
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");


    let client = reqwest::Client::new();
    let mut post_response = client.post("http://httpbin.org/post")
        .json(&map)
        .send()?;
    println!("{:?}", post_response.text()?);



    return Ok(())
}

