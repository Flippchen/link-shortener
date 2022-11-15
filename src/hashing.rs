use std::collections::HashMap;
use random_string::generate;
use std::fs;
use std::fs::File;
use std::io::Write;
use url::Url;

pub fn generate_hash() -> String {
    // TODO: Take hashing algorithm
    let charset = "abcdefghijklmnopqrstuvwxyz1234567890";
    generate(10, charset)
}


pub fn load_hashmap() -> HashMap<String, String> {
    let file = fs::read_to_string("links.toml")
        .expect("Could not find links.toml");
    let deserialized = toml::from_str(file.as_str()).expect("Not serializeable");
    deserialized
}

fn save_hashmap(names: HashMap<String, String>) {
    let serialized = toml::to_string(&names).unwrap();
    let path = "links.toml";
    let mut file = File::create(path).unwrap();
    write!(file, "{}", serialized).expect("Could not write to file");
}

pub fn add_link(code: String, input_name: String) {
    let mut hashmap = load_hashmap();
    hashmap.insert(code, input_name.to_string());

    save_hashmap(hashmap);
}

pub fn load_link(code: String) -> String {
    let hashmap = load_hashmap();
    match hashmap.get(code.as_str()) {
        None => "Not found".to_string(),
        Some(selected_value) => selected_value.to_string(),
    }
}