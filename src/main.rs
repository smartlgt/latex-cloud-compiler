use reqwest;
use reqwest::blocking::multipart;
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

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: cloudtex path-to-tex-file");
        return
        
    }


    // check if relative or full path was given
    let mut filepath = Path::new( &args[1]);

    let mut new_path;
    let new_path2;

    if filepath.is_relative() {
            new_path = env::current_dir().unwrap();
            // put the relativ path ontop of the file eecut
            new_path.push(filepath);
            filepath = new_path.as_path();
            new_path2 = filepath.canonicalize().unwrap();
            filepath = new_path2.as_path();
    }

    if !filepath.exists() || !filepath.is_file() {
        println!("{} is no a file or deose not exist", filepath.display());
        return
    }

    let dirpath = filepath.parent().unwrap();
    let filename = filepath.file_name().unwrap();

    println!("start cloud tex compiler for: {}", filepath.display());

    let mut form = multipart::Form::new();

    let fls = filename.to_string_lossy();
    form = form.file(String::from(fls), filepath).unwrap();

    
    
    let walker = WalkDir::new(dirpath).max_depth(2).into_iter();

    let mut counter = 1;
    let mut total_size = 0;
    for entry in walker.filter_entry(|e| !is_hidden(e)) {

        
        counter = counter + 1;
        
        //let entryn = entry.unwrap();

        let p = entry.unwrap();
        let pp = p.path();

        // skip the main file
        if filepath == pp {
            continue
        }

        let metadata = fs::metadata(pp).unwrap();
        if metadata.is_file() {

            total_size += metadata.len();

            //println!("{}", pp.display());
            form = form.file(String::from(pp.to_str().unwrap()), pp).unwrap();


        }

    }



    println!("transfer total size: {} bytes", total_size);
    let total_size_mb = total_size as f64 / 1024.0 / 1024.0;
    println!("transfer total size: {:.2} mb", total_size_mb);
    



    let client = reqwest::blocking::Client::new();



    let req = client
    .post("http://127.0.0.1:5000/")
    .header(AUTHORIZATION, "changeme")
    .multipart(form);   
    
    let resp = req.send().unwrap();
    //println!("{:#?}", resp);

    println!("cloud status response code: {}", resp.status());

    let code = resp.status();
    if code.is_success() {
        // response is a single pdf
        let body = resp.bytes().unwrap();
        //println!("{:#?}", body);

        let output_name = Path::new(filename).file_stem().unwrap();

        let mut file = File::create(format!("{}.{}", output_name.to_string_lossy(), "pdf")).expect("error creating file");
        file.write_all(&body).expect("error writing output file");

        std::process::exit(0);

    } else if code.as_u16() == 422 {
        // latex compile error, body containst the error log
        let body = resp.text().unwrap();

        eprintln!("{}", body);
        std::process::exit(1);

    }
    
    
    eprintln!("something went wrong");
    std::process::exit(1);
    
    
}
