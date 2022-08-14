use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{cell::RefCell, error::Error, fs, path::Path};

#[derive(Clone, Serialize)]
pub struct Item {
    // Representation of an item which is a symbol+name
    pub rep: String,

    // Type of an item
    pub r#type: String,

    // A string path in order to access a specific Item in a Tree
    pub obj_ref: String,
}

pub struct FolderTree {
    // List of items that hold a representation string and an object path
    pub items: RefCell<Vec<Item>>,

    // Original object containing all the values
    pub raw_data: Value,

    // path reference to update the file
    path: String,
}

// This is used only to serialize new folder which will be inserted
#[derive(Deserialize, Serialize)]
struct NewFolder {
    r#type: String,
    name: String,
    folded: bool,
    path: String,
    items: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct NewEndpoint {
    r#type: String,
    name: String,
    method: String,
    path: String,
    url: String,
    json_body: String,
}

impl FolderTree {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let input = fs::read_to_string(path.as_ref())?;
        let raw_data = serde_json::from_str(input.as_str())?;
        let items = RefCell::new(Vec::new());

        let path = String::from(path.as_ref().to_str().unwrap());

        Ok(FolderTree {
            items,
            raw_data,
            path,
        })
    }

    pub fn parse_all(&self) {
        // Each time we re-parse everything we have to clear the items vector
        self.items.borrow_mut().clear();

        let json_data = self.raw_data.get("root").and_then(Value::as_array).unwrap();

        self.parse(json_data, 0);
    }

    fn parse(&self, json_data: &[Value], indent: i32) {
        let mut temp_vec: Vec<Item> = Vec::new();

        for data in json_data.iter() {
            let val = Value::deserialize(data).unwrap();

            match val.get("type").and_then(Value::as_str).unwrap() {
                "endpoint" => {
                    self.parse_endpoint(&val, indent);
                }
                "folder" => {
                    let is_folded = val.get("folded").and_then(Value::as_bool).unwrap();
                    let indented = construct_indent(indent);
                    let name = val.get("name").and_then(Value::as_str).unwrap();

                    let symbol = if is_folded { "▸" } else { "▾" };

                    let new_item = Item {
                        rep: format!("{}{} 📁 {}", indented, symbol, name),
                        r#type: val.get("type").and_then(Value::as_str).unwrap().to_string(),
                        obj_ref: val.get("path").and_then(Value::as_str).unwrap().to_string(),
                    };

                    temp_vec.push(new_item);

                    self.items.borrow_mut().append(&mut temp_vec);

                    if !is_folded {
                        self.parse_folder(&val, false, indent + 1);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn parse_endpoint(&self, val: &Value, indent: i32) {
        let cur_endpoint: NewEndpoint = serde_json::from_value(val.clone()).unwrap();

        let ind = construct_indent(indent);

        let temp_obj: Item = Item {
            rep: format!("{}  {} {}", ind, cur_endpoint.method, cur_endpoint.name),
            r#type: String::from("endpoint"),
            obj_ref: val.get("path").and_then(Value::as_str).unwrap().to_string(),
        };

        self.items.borrow_mut().push(temp_obj);
    }

    // Find the closest folder that we can insert out new thing into
    fn find_closest_folder(&self, path: &str) -> String {
        let mut is_folder = false;
        let mut split = path.split('/').collect::<Vec<&str>>();
        let mut ptr = self.raw_data.pointer(path).unwrap();

        while is_folder == false {
            if ptr.get("items").is_some() {
                is_folder = true;
            } else {
                split.remove(split.len() - 1);
                ptr = self.raw_data.pointer(split.join("/").as_str()).unwrap();
            }
        }

        split.join("/")
    }

    fn build_new_path(&self, path: &str) -> String {
        let mut split: Vec<&str> = path.split('/').collect();

        let new_split = split[split.len() - 1];

        let parsed = new_split.parse::<i32>().unwrap();

        let parsed_plusone = (parsed + 1).to_string();

        split.remove(split.len() - 1);
        split.push(parsed_plusone.as_str());

        split.join("/")
    }

    pub fn current_endpoint(&self, path: &str) -> Option<Map<String, Value>> {
        if let Some(current) = self.raw_data.pointer(path) {
            if current.get("type") == Some(&Value::String(format!("endpoint"))) {
                return Some(current.as_object().unwrap().to_owned());
            }
            None
        } else {
            None
        }
    }

    pub fn can_fold_folder(&self, path: &str) -> bool {
        let check = self.raw_data.pointer(path).unwrap().as_object().unwrap();

        if check["type"] == Value::String(String::from("folder"))
            && check["folded"] == Value::Bool(false)
        {
            return true;
        }

        false
    }

    pub fn can_unfold_folder(&self, path: &str) -> bool {
        let check = self.raw_data.pointer(path).unwrap().as_object().unwrap();

        if check["type"] == Value::String(String::from("folder"))
            && check["folded"] == Value::Bool(true)
        {
            return true;
        }

        false
    }

    pub fn fold_folder(&mut self, path: &str) {
        // do not unwrap but check if its actually a folder
        let check = self
            .raw_data
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();

        if check["type"] == Value::String(String::from("folder"))
            && check["folded"] == Value::Bool(false)
        {
            check["folded"] = Value::Bool(true);

            self.parse_all();
        }
    }

    pub fn unfold_folder(&mut self, path: &str) {
        // do not unwrap but check if its actually a folder
        let check = self
            .raw_data
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();

        if check["type"] == Value::String(String::from("folder"))
            && check["folded"] == Value::Bool(true)
        {
            check["folded"] = Value::Bool(false);

            self.parse_all();
        }
    }

    pub fn insert_endpoint(&mut self, path: &str, name: &str) {
        let new_path = self.build_new_path(path);
        let closest_folder = self.find_closest_folder(path);

        let endpoint = NewEndpoint {
            r#type: String::from("endpoint"),
            name: String::from(name),
            method: String::from("POST"),
            path: new_path,
            url: String::from("TODO"),
            json_body: String::from("TODO"),
        };

        let data_pointer = self
            .raw_data
            .pointer_mut(&closest_folder)
            .unwrap()
            .get_mut("items")
            .and_then(Value::as_array_mut)
            .unwrap();

        let j = serde_json::to_string(&endpoint).unwrap();
        let k: Value = serde_json::from_str(j.as_str()).unwrap();

        data_pointer.push(k);
        self.parse_all();
        self.update_file();
    }

    fn update_file(&self) {
        let s = self.path.as_str();
        let p = Path::new(s);
        let mut f = fs::File::create(p).unwrap();

        let xd: String = serde_json::to_string_pretty(&self.raw_data).unwrap();

        use std::io::prelude::*;

        f.write_all(xd.as_bytes()).unwrap();
    }

    pub fn parse_folder(&self, val: &Value, _folded: bool, indent: i32) {
        let arr = val.get("items").and_then(Value::as_array).unwrap();

        self.parse(arr, indent);
    }
}

fn construct_indent(indent: i32) -> String {
    let mut ind = String::from("");
    for _ in 0..indent {
        ind.push_str("    ");
    }

    ind
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use tempfile::NamedTempFile;

    fn initialize() -> FolderTree {
        let input = r#"
            {
                "root": [
                    {
                        "type": "folder",
                        "name": "Pierwszy",
                        "folded": false,
                        "path": "/root/0",
                        "items": [
                            {
                                "type": "endpoint",
                                "name": "Dodaj usera",
                                "method": "POST",
                                "path": "/root/0/items/0",
                                "url": "http://localhost:3000/1"
                            },
                            {
                                "type": "endpoint",
                                "name": "Zmien userow",
                                "method": "PUT",
                                "path": "/root/0/items/1",
                                "url": "http://localhost:3000/2"
                            },
                            {
                                "type": "folder",
                                "name": "Nested",
                                "folded": false,
                                "path": "/root/0/items/2",
                                "items": [
                                    {
                                        "type": "endpoint",
                                        "name": "Nested jeszcze",
                                        "method": "GET",
                                        "path": "/root/0/items/2/items/0",
                                        "url": "http://localhost:3000/2"
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "endpoint",
                        "name": "Costam",
                        "method": "POST",
                        "path": "/root/1",
                        "url": "http://localhost:3000/4"
                    },
                    {
                        "type": "folder",
                        "name": "Trzeci folder",
                        "folded": false,
                        "path": "/root/2",
                        "items": []
                    }
                ]
            }
        "#;

        let mut file = NamedTempFile::new().unwrap();

        file.write_all(input.as_bytes()).unwrap();

        FolderTree::new(file.path()).unwrap()
    }

    #[test]
    fn test_build_new_path_on_folder() {
        let ft = initialize();

        assert_eq!(ft.build_new_path("/root/0"), String::from("/root/1"));
    }

    #[test]
    fn test_build_new_path_on_endpoint() {
        let ft = initialize();

        assert_eq!(
            ft.build_new_path("/root/0/items/1"),
            String::from("/root/0/items/2")
        );
    }

    #[test]
    fn test_find_closest_folder_on_folder() {
        let ft = initialize();

        assert_eq!(
            ft.find_closest_folder("/root/0/items/2"),
            String::from("/root/0/items/2")
        );
    }

    #[test]
    fn test_find_closest_folder_on_endpoint() {
        let ft = initialize();

        assert_eq!(
            ft.find_closest_folder("/root/0/items/2/items/0"),
            String::from("/root/0/items/2")
        );
    }
}
