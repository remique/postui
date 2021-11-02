use serde::Deserialize;

#[derive(Deserialize)]
pub struct Endpoint {
    pub method: String,
    pub name: String,
    pub r#type: String,
}

pub struct Item {
    pub rep: String,
    pub obj: serde_json::Value,
}

pub struct FolderTreeComponent {
    pub items: Vec<Item>,
}

impl FolderTreeComponent {
    pub fn new() -> Self {
        FolderTreeComponent { items: Vec::new() }
    }

    pub fn from_str(&mut self, input: &str) {
        let json_data: Vec<serde_json::Value> = serde_json::from_str(input).unwrap();
        let indent: i32 = 0;

        self.parse(&json_data, indent);
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
                        obj: val.clone(),
                    };

                    temp_vec.push(temp_obj);

                    // temp_vec.push(String::from(format!(
                    //     "{}{} {}",
                    //     construct_indent(indent),
                    //     symbol,
                    //     val.get("name").and_then(serde_json::Value::as_str).unwrap()
                    // )));

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
        let des: Endpoint = serde_json::from_value(val.clone()).unwrap();

        let ind = construct_indent(indent);

        let temp_obj: Item = Item {
            rep: String::from(format!("{}  {} {}", ind, des.method, des.name)),
            obj: val.clone(),
        };

        self.items.push(temp_obj);

        // self.items.push(String::from(format!(
        //     "{}  {} {}",
        //     ind, des.method, des.name
        // )));
    }

    pub fn parse_folder(&mut self, val: &serde_json::Value, _folded: bool, indent: i32) {
        let arr = val
            .get("items")
            .and_then(serde_json::Value::as_array)
            .unwrap();

        self.parse(arr, indent);
    }
}

fn construct_indent(indent: i32) -> String {
    let mut ind = String::from("");
    for _ in 0..indent {
        ind.push_str("  ");
    }

    ind
}

// So each item need some kind of indentation info? Or We have to
// keep the information where this item is in order to change some value
//
// NEW: I guess we have to use derive(Serialize) and serialize an entire object
// and then parse it? Which would give us access to specific fields
// and we could easily serialize/deserialize everything
//
// I guess we can keep serde_json::Value inside the structure field and then
// parsing that is easy?
// Reference: https://github.com/serde-rs/json/issues/144#issuecomment-242877324
//
// Another reference because I need an anonymous array instead of single object
// https://newbedev.com/how-can-i-use-serde-with-a-json-array-with-different-objects-for-successes-and-errors
//
// Another one.... From the author, might be good. Also come back to the second link
// https://www.reddit.com/r/rust/comments/7hasv6/mixed_valuestruct_deserialization_with_serde_json/?sort=top
