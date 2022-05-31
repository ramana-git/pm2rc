use std::{fs, path::Path};
use pm2rc::{Collection,Folder,Item,EndPoint};

fn main()-> Result<(), serde_json::Error>{
    let args: Vec<String> = std::env::args().collect();
    if args.len()!=2 {
        println!("Usage:\r\n{} <Postman-Collection-Filename>\r\n",&args[0]);
        return Ok(())
    }
    let str_json=fs::read_to_string(&args[1]).unwrap();
    let collection:Collection=serde_json::from_str(&str_json)?;
    create_directory(&collection.info.name);
    save_items(&collection.info.name,&collection.items);
    println!("{:?}",collection);
    Ok(())
}

fn create_directory(name:&str){
    if Path::new(name).is_dir(){
        panic!("There is an existing test folder: {}",name);
    }
    fs::create_dir(name).expect("Unable to create directory");
}

fn save_items(directory:&str, items:&Vec<Item>){
    let filename=directory.to_owned()+".rest";
    let mut contents=String::new();
    for item in items{
        match item {
            Item::Path(Folder { name, items })=>{
                save_items(&(directory.to_owned()+"/"+&name), items);
            }
            Item::EndPoint(EndPoint { name, request, response: _ })=>{
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
        }
    }
    if contents.len()>0 {
        fs::write(filename, contents).unwrap();
    }
}