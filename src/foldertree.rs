use serde::Deserialize;

#[derive(Deserialize)]
pub struct Endpoint {
    pub method: String,
    pub name: String,
    pub r#type: String,
}

pub struct Item {
    // Representation of an item which is a symbol+name
    pub rep: String,

    // A string path in order to access a specific Item in a Tree
    pub obj_ref: String,
}

pub struct FolderTree {
    pub items: Vec<Item>,
    pub data: serde_json::Value,
}

pub struct InnerItem {
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub method: Option<String>,
    pub folded: Option<bool>,
    pub items: Option<Vec<InnerItem>>,
    pub path: Option<String>,
}

impl FolderTree {
    pub fn new() -> Self {
        FolderTree {
            items: Vec::new(),
            data: serde_json::Value::Null,
        }
    }

    pub fn load_str(&mut self, input: &str) {
        let original_input: serde_json::Value = serde_json::from_str(input).unwrap();
        let element_list = original_input
            .get("root")
            .and_then(serde_json::Value::as_array)
            .unwrap();

        self.parse(element_list, 0);
    }

    pub fn parse(&mut self, json_data: &Vec<serde_json::Value>, indent: i32) {
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

                    self.items.append(&mut temp_vec);

                    if !is_folded {
                        self.parse_folder(&val, false, indent + 1);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn parse_endpoint(&mut self, val: &serde_json::Value, indent: i32) {
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

        self.items.push(temp_obj);
    }

    pub fn parse_folder(&mut self, val: &serde_json::Value, _folded: bool, indent: i32) {
        let arr = val
            .get("items")
            .and_then(serde_json::Value::as_array)
            .unwrap();

        self.parse(arr, indent);
    }

    pub fn generate_indices(&self) {
        let new_vec: Vec<Vec<InnerItem>> = Vec::new();

        for (idx, item) in self.items.iter().enumerate() {
            println!("{} -- {:?}\n", idx, item.rep);
        }
    }

    fn get_element_ptr(&self, path: &str) -> Option<&serde_json::Value> {
        let ptr = self.data.pointer(path);

        ptr
    }

    fn get_element_ptr_mut(&mut self, path: &str) -> Option<&mut serde_json::Value> {
        let ptr = self.data.pointer_mut(path);

        ptr
    }
}

fn construct_indent(indent: i32) -> String {
    let mut ind = String::from("");
    for _ in 0..indent {
        ind.push_str("  ");
    }

    ind
}
