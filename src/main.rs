use reqwest;
use reqwest::blocking::multipart;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use reqwest::header::AUTHORIZATION;

use walkdir::{DirEntry, WalkDir};

use std::env;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}


fn main() {
    let mut form = multipart::Form::new();

    let path = env::current_dir().unwrap();
    
    let walker = WalkDir::new(Path::new(&path)).max_depth(2).into_iter();

    let mut counter = 1;
    let mut total_size = 0;
    for entry in walker.filter_entry(|e| !is_hidden(e)) {

        
        counter = counter + 1;
        
        //let entryn = entry.unwrap();

        let p = entry.unwrap();
        let pp = p.path();

        let metadata = fs::metadata(pp).unwrap();
        if (metadata.is_file()) {

            total_size += metadata.len();
            
            //println!("{:?}", metadata.file_type());
            println!("{}", pp.display());


            let p2 = pp.to_str();
            let x = p2.unwrap();

            form = form.file(counter.to_string(), pp).unwrap();
            //let bio = multipart::Part::file(pp).unwrap().file_name(x);

            //form = form.part(x, bio);
            //form = form.text("", "");


        }

        /*

        let metadata = fs::metadata(&pp).unwrap();
        if (metadata.is_file()) {

            println!("{:?}", metadata.file_type());

            //println!("{}", entry.path().display());

            let p = &pp.to_str();
            
                let x = p.unwrap();

                form = form.file(x, pp).unwrap();
            
        }*/

    }



    println!("transfer total size: {} bytes", total_size);
    let total_size_mb = total_size as f64 / 1024.0 / 1024.0;
    println!("transfer total size: {:.2} mb", total_size_mb);
    



    let client = reqwest::blocking::Client::new();



    let mut req = client
    .post("http://127.0.0.1:5000/")
    .header(AUTHORIZATION, "changeme")
    .multipart(form);

    
    
    
    
    let resp = req.send();


    
    println!("{:#?}", resp);


    let body = resp.unwrap().bytes().unwrap();
    //println!("{:#?}", body);

    let mut file = File::create("output.pdf").expect("error creating file");
    file.write_all(&body);

}


    
    /*
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("key3", "value3")
        .file("file", "/path/to/field")?;
    
    let response = client.post("your url")
        .multipart(form)
        .send()?;



    let bio = multipart::Part::text("main.tex")
    .file_name("main.tex")
    .mime_str("text/plain").expect("error loading file");

    let form = multipart::Form::new();

    form.await?.file("main.tex", "test.tex");

    // Add the custom part to our form...
    let form_part = form.part("main.tex", bio);




    // And finally, send the form
    let client = reqwest::Client::new();
    let resp = client
        .post("http://127.0.0.1:5000/")
        .multipart(form_part)
        .send()
        .await?;

    println!("{:#?}", resp);

    let body = resp.text().await?;    
    println!("{:#?}", body);


    Ok(())
    

*/