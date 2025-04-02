use std::{fs, path::Path};
use pm2rc::{Collection, Folder, Item, HttpRequest};

fn main()-> Result<(), serde_json::Error>{
    let args: Vec<String> = std::env::args().collect();
    if args.len()!=2 {
        println!("Usage:\r\n{} <Postman-Collection-Filename>\r\n",&args[0]);
        return Ok(())
    }
    let file_path = &args[1];
    if !Path::new(file_path).exists() {
        println!("File not found: {}", file_path);
        return Ok(())
    }
    let str_json=fs::read_to_string(file_path).unwrap();
    let collection:Collection=serde_json::from_str(&str_json)?;

    if Path::new(&collection.info.name).is_dir() {
        println!("Directory already exists: {}", &collection.info.name);
        return Ok(());
    }

    save_items(&collection.info.name,&collection.items);
    Ok(())
}

fn save_items(directory:&str, items:&Vec<Item>){
    let filename=directory.to_owned()+".rest";
    let mut contents=String::new();
    for item in items{
        match item {
            Item::Folder(Folder { name, items })=>{
                let sub_directory=format!("{}/{}",directory,name);
                save_items(&sub_directory, items);
            }
            Item::HttpRequest(HttpRequest { name, request})=>{
                let mut bytes=format!("# @{}\r\n{} {}\r\n",name,request.method,request.url.raw);
                if let Some(headers) = &request.headers {
                    for header in headers{
                        bytes.push_str(&format!("{}: {}\r\n",header.key,header.value));
                    }
                }
                bytes.push_str("\r\n");
                if let Some(body)= &request.body {
                    bytes.push_str(&body.raw);
                }
                bytes.push_str("\r\n\r\n###\r\n");
                contents.push_str(&bytes);
            }
            Item::Unknown(value) => {eprintln!("Unknown Item: {:?}", value);}
        }
    }
    if !contents.is_empty() {
        if let Some(parent_dir) = Path::new(&filename).parent() {
            fs::create_dir_all(parent_dir).expect("Unable to create directory");
        }
        fs::write(filename, contents).unwrap();
    }
}