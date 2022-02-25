use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;

#[derive(Deserialize)]
pub struct Endpoint {
    pub method: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Clone)]
pub struct Item {
    // Representation of an item which is a symbol+name
    pub rep: String,

    // A string path in order to access a specific Item in a Tree
    pub obj_ref: String,
}

pub struct FolderTree {
    // List of items that hold a representation string and an object path
    pub items: RefCell<Vec<Item>>,

    // Original object containing all the values
    pub data: serde_json::Value,
}

// This is used only to serialize new folder which will be inserted
#[derive(Serialize)]
struct NewFolder {
    r#type: String,
    name: String,
    folded: bool,
    path: String,
    items: Vec<String>,
}

#[derive(Serialize)]
struct NewEndpoint {
    r#type: String,
    name: String,
    method: String,
    path: String,
}

impl FolderTree {
    pub fn from_str(input: &str) -> Self {
        FolderTree {
            items: RefCell::new(Vec::new()),
            data: serde_json::from_str(input).unwrap(),
        }
    }

    pub fn parse_all(&self) {
        // Each time we re-parse everything we have to clear the items vector
        self.items.borrow_mut().clear();

        let json_data = self
            .data
            .get("root")
            .and_then(serde_json::Value::as_array)
            .unwrap();

        self.parse(json_data, 0);
    }

    fn parse(&self, json_data: &Vec<serde_json::Value>, indent: i32) {
        let mut temp_vec: Vec<Item> = Vec::new();

        for data in json_data.iter() {
            let val = serde_json::Value::deserialize(data).unwrap();

            match val.get("type").and_then(serde_json::Value::as_str).unwrap() {
                "endpoint" => {
                    self.parse_endpoint(&val, indent);
                }
                "folder" => {
                    let is_folded = val
                        .get("folded")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap();

                    let symbol = if is_folded {
                        String::from("▸")
                    } else {
                        String::from("▾")
                    };

                    let temp_obj: Item = Item {
                        rep: String::from(format!(
                            "{}{} {}",
                            construct_indent(indent),
                            symbol,
                            val.get("name").and_then(serde_json::Value::as_str).unwrap()
                        )),
                        obj_ref: val
                            .get("path")
                            .and_then(serde_json::Value::as_str)
                            .unwrap()
                            .to_string(),
                    };

                    temp_vec.push(temp_obj);

                    self.items.borrow_mut().append(&mut temp_vec);

                    if !is_folded {
                        self.parse_folder(&val, false, indent + 1);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn parse_endpoint(&self, val: &serde_json::Value, indent: i32) {
        let cur_endpoint: Endpoint = serde_json::from_value(val.clone()).unwrap();

        let ind = construct_indent(indent);

        let temp_obj: Item = Item {
            rep: String::from(format!(
                "{}  {} {}",
                ind, cur_endpoint.method, cur_endpoint.name
            )),
            obj_ref: val
                .get("path")
                .and_then(serde_json::Value::as_str)
                .unwrap()
                .to_string(),
        };

        self.items.borrow_mut().push(temp_obj);
    }

    fn build_path_insert(&self, path: &str) -> String {
        // We split the path into a vector of '/'
        let tmp: Vec<&str> = path.split('/').collect();
        let tmp_data = self.data.pointer(path).unwrap();

        // This looks weird but works for now
        let mut new;

        if tmp_data.get("items").is_some() {
            new = tmp.join("/");

            if tmp_data
                .get("items")
                .and_then(serde_json::Value::as_array)
                .unwrap()
                .len()
                == 0
            {
                // We append /items/0 as it is the first item in the array
                new.push_str(format!("/items/{}", 0).as_str());
            } else {
                let index = self
                    .data
                    .pointer(format!("{}/items", new.as_str()).as_str())
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .len();

                new.push_str(format!("/items/{}", index.to_string().as_str()).as_str());
            }
        } else {
            new = tmp[0..tmp.len() - 1].join("/");
            let index = self
                .data
                .pointer(new.as_str())
                .unwrap()
                .as_array()
                .unwrap()
                .len();
            new.push_str(format!("/{}", index.to_string().as_str()).as_str());
        }

        new
    }

    fn get_truncated_path(&self, path: &str) -> String {
        let tmp_split: Vec<&str> = path.split('/').collect();
        let previous_folder_path = tmp_split[0..tmp_split.len() - 1].join("/");

        previous_folder_path
    }

    pub fn fold_folder(&mut self, path: &str) {
        // do not unwrap but check if its actually a folder
        let check = self
            .data
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();

        // Fix
        if check["folded"] == serde_json::Value::Bool(false) {
            check["folded"] = serde_json::Value::Bool(true);

            self.parse_all();
        }
    }

    pub fn unfold_folder(&mut self, path: &str) {
        // do not unwrap but check if its actually a folder
        let check = self
            .data
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();

        // Fix
        if check["folded"] == serde_json::Value::Bool(true) {
            check["folded"] = serde_json::Value::Bool(false);

            self.parse_all();
        }
    }

    pub fn insert_folder(&mut self, path: &str, name: &str) {
        let new_path_tmp = self.build_path_insert(path);

        let newfolder = NewFolder {
            r#type: String::from("folder"),
            name: String::from(name),
            folded: false,
            path: new_path_tmp,
            items: vec![],
        };

        let previous_folder_path = self.get_truncated_path(path);

        let dts = self
            .data
            .pointer_mut(previous_folder_path.as_str())
            .unwrap()
            .as_array_mut()
            .unwrap();

        let j = serde_json::to_string(&newfolder).unwrap();
        let k: serde_json::Value = serde_json::from_str(j.as_str()).unwrap();

        dts.push(k);
        self.parse_all();
    }

    pub fn insert_endpoint(&mut self, path: &str, name: &str) {
        let new_path_tmp = self.build_path_insert(path);

        let newendpoint = NewEndpoint {
            r#type: String::from("endpoint"),
            name: String::from(name),
            method: String::from("POST"),
            path: new_path_tmp,
        };

        let dts = self
            .data
            .pointer_mut(path)
            .unwrap()
            .get_mut("items")
            .and_then(serde_json::Value::as_array_mut)
            .unwrap();

        let j = serde_json::to_string(&newendpoint).unwrap();
        let k: serde_json::Value = serde_json::from_str(j.as_str()).unwrap();

        dts.push(k);
        self.parse_all();
    }

    pub fn parse_folder(&self, val: &serde_json::Value, _folded: bool, indent: i32) {
        let arr = val
            .get("items")
            .and_then(serde_json::Value::as_array)
            .unwrap();

        self.parse(arr, indent);
    }

    pub fn show_representation(&self) {
        for item in self.items.borrow().iter() {
            println!("{:?}", item.rep);
        }
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

    fn setup_ft() -> FolderTree {
        let k = r#"
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
                                "path": "/root/0/items/0"
                            },
                            {
                                "type": "endpoint",
                                "name": "Zmien userow",
                                "method": "PUT",
                                "path": "/root/0/items/1"
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
                                        "path": "/root/0/items/2/items/0"
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "type": "endpoint",
                        "name": "Costam",
                        "method": "POST",
                        "path": "/root/1"
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

        FolderTree::from_str(k)
    }

    #[test]
    fn test_path_new_folder_to_endpoint() {
        let ft = setup_ft();

        assert_eq!(
            String::from("/root/0/items/3"),
            ft.build_path_insert("/root/0/items/1")
        );
    }

    #[test]
    fn test_path_new_folder_to_empty_folder() {
        let ft = setup_ft();

        assert_eq!(
            String::from("/root/2/items/0"),
            ft.build_path_insert("/root/2")
        );
    }

    #[test]
    fn test_path_new_folder_to_non_empty_folder() {
        let ft = setup_ft();

        assert_eq!(
            String::from("/root/0/items/2/items/1"),
            ft.build_path_insert("/root/0/items/2")
        );
    }

    #[test]
    fn test_basic_folder_insertion() {
        let mut ft = setup_ft();

        ft.insert_folder("/root/0/items/1", "new folder");

        assert_eq!(
            ft.data
                .pointer("/root/0/items/3")
                .unwrap()
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap(),
            "new folder"
        );
    }

    #[test]
    fn test_basic_endpoint_insertion() {
        let mut ft = setup_ft();

        ft.insert_endpoint("/root/0/items/2", "new endpoint");

        assert_eq!(
            ft.data
                .pointer("/root/0/items/2/items/1")
                .unwrap()
                .get("name")
                .and_then(serde_json::Value::as_str)
                .unwrap(),
            "new endpoint"
        );
    }
}
