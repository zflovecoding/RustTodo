use std::str::FromStr;
use std::{collections::HashMap, fmt::format, io::Read};

fn main() {
    let action = std::env::args().nth(1).expect("please specify an action");
    let item = std::env::args().nth(2).expect("please specify an item");
    println!("{:?},{:?}", action, item);
    let mut todo = Todo::new().expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

struct Todo {
    //使用 Rust 内置的 HashMap 来保存 key - val 键值对。
    map: HashMap<String, bool>,
}
impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        // 打开 db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        // 序列化 json 为 HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        // 在我们的 map 中新增一个新的元素。
        // 我们默认将其状态值设置为 true
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // 打开 db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // 通过 Serde 写入文件
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
