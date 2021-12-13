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

    pub fn insert_folder(&mut self, path: &str) {
        // We get element ptr to the path
        //
        // We check if we got a folder or an endpoint
        // If we got an endpoint we have to subract the string as to add to the folder
        // If we got a folder in the path we do nothing
        //
        // Then we have to prepare a new serde_json::Value compliant to the endpoint/folder look
        // We have to generate a path for it and then push it to the vector of items and update
        // self.data (borrow as mut) as in example in main.rs

        // EXAMPLE FROM MAIN:

        // S should serialize normally via serde_json:
        //     let s = r#"
        //         {
        //             "type": "endpoint",
        //             "name": "Dodany na nowo",
        //             "method": "POST""#;
        // let mut data: serde_json::Value = serde_json::from_str(k).unwrap();

        // // println!("{:?}", data);
        // // println!("{:#?}", data.pointer("/root/0/items/1").unwrap());

        // // TODO Better construction of "s"
        // // Can be done through normal serialization i guess:=)
        // let new_path = "/root/0/items/1".trim_end_matches("/1");
        // let mut new_ptr = data.pointer_mut(new_path).unwrap().as_array_mut().unwrap();

        // let mut new_path_to_save = String::from(new_path);
        // let idx_new = new_ptr.len();
        // new_path_to_save.push_str(format!("/{}", idx_new).as_str());
        // let mut ss = String::from(s);
        // ss.push_str(",\n \"path\": \"");
        // ss.push_str(new_path_to_save.as_str());
        // ss.push_str("\"\n}");
        // let new_data: serde_json::Value = serde_json::from_str(ss.as_str()).unwrap();
        // // println!("{}", new_path);
        // new_ptr.push(new_data);
        // println!("{:#?}", data);
        // // and then push into the vector of items another value
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
        // I believe we should wrap the self.data into a RefCell<T> and then borrow it
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
