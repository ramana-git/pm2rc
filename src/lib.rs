use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Info{
    pub name: String,
    pub schema: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Item{
    Folder(Folder),
    HttpRequest(HttpRequest),
    Unknown(serde_json::Value)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder{
    pub name: String,
    #[serde(rename = "item")]
    pub items: Vec<Item>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HttpRequest {
    pub name: String,
    pub request: Request
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header{
    pub key: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct URL{
    pub raw: String,
    pub protocol: String,
    pub host: Vec<String>,
    pub port: Option<String>,
    pub path: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body{
    pub mode: String,
    pub raw: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request{
    pub method:String,
    #[serde(rename = "header")]
    pub headers:Option<Vec<Header>>,
    pub body:Option<Body>,
    pub url:URL
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response{
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection{
    pub info: Info,
    #[serde(rename = "item")]
    pub items: Vec<Item>
}
