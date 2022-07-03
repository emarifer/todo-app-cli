use chrono::{Local, Timelike, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Todo {
    // Usamos el tipo HashMap que está incorporado en Rust.
    pub map: HashMap<String, (bool, String)>,
}

impl Todo {
    pub fn new(folder: &String) -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(format!("{}/.db.json", &folder[..]))?; // VER NOTA-1 ABAJO:

        // Serializar el archivo json como HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("Ha ocurrido un error: {} 😱", e),
        }
    }

    pub fn insert(&mut self, key: String) {
        // Insertamos un nuevo valor en nuestro mapa.
        // Por default, el value va a ser true.
        self.map
            .insert(key.trim().to_string(), (true, get_time_and_date()));
    }

    pub fn save(self, folder: &String) -> Result<(), Box<dyn std::error::Error>> {
        // Abrir db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true) // VER NOTA-1 ABAJO:
            .create(true)
            .open(format!("{}/.db.json", &folder[..]))?;

        // Escribir en el archivo con serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    pub fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key.trim()) {
            Some(v) => Some(*v = (false, get_time_and_date())),
            None => None,
        }
    }

    pub fn delete(&mut self, key: &String) -> Option<String> {
        match self.map.remove_entry(key.trim()) {
            Some(e) => Some(e.0),
            None => None,
        }
    }
    // pub fn delete(&mut self, key: &String) {
    //     self.map.retain(|k, _| k != key.trim());
    // }
}

fn get_time_and_date() -> String {
    let now = Local::now();
    let date = Utc::now().date().format("%d-%m-%Y");
    let (is_pm, hour) = now.hour12();
    format!(
        "{:02}:{:02}:{:02} {} • {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" },
        date
    )
}

/*
 * NOTA-1
 * SOBRE ESCRIBIR ARCHIVO AL ELIMINAR UN ITEM DEL HASHMAP:
 * https://ddanilov.me/how-to-overwrite-a-file-in-rust
 */
