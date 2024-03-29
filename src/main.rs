use std::collections::HashMap;

fn main() {
    let action: String = std::env::args().nth(1).expect("Please specify an action");
    let item: String = std::env::args().nth(2).expect("Please specify an item");

    let mut todo: Todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Saved!"),
            Err(err) => println!("Error: {}", err),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Saved!"),
                Err(err) => println!("Error: {}", err),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f: std::fs::File = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("db.json")?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(err) if err.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(err) => panic!("An error occurred: {}", err),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f: std::fs::File = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
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
