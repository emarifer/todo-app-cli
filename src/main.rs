use std::collections::HashMap;
// use std::io::Read;
// use std::str::FromStr;

fn main() {
    let mut user_home = String::new();

    match home::home_dir() {
        Some(path) => match path.to_str() {
            None => println!("¡Imposible conseguir el directorio Home! 😱"),
            Some(my_home) => user_home = my_home.to_string(),
        },
        None => println!("¡Imposible conseguir el directorio Home! 😱"),
    }

    // println!("Mi carpeta de usuario es: {}", user_home);

    let action = std::env::args()
        .nth(1)
        .expect("Por favor, especifica una acción");

    let item = std::env::args()
        .nth(2)
        .expect("Por favor, especifica un item");

    let mut todo = Todo::new(&user_home).expect("La inicialización de la db falló 😱");

    if action == "add" {
        todo.insert(item);

        match todo.save(&user_home) {
            Ok(_) => println!("Todo guardado correctamente 😀"),
            Err(why) => println!("Ha ocurrido un error: {} 😱", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' no está presente en la lista de ToDos", item),
            Some(_) => match todo.save(&user_home) {
                Ok(_) => println!("Todo actualizado"),
                Err(why) => println!("Ha ocurrido un error: {} 😱", why),
            },
        }
    } else {
        println!("No es una acción válida")
    }

    // println!("{:?}, {:?}", action, item);
}

struct Todo {
    // Usamos el tipo HashMap que está incorporado en Rust.
    map: HashMap<String, bool>,
}

impl Todo {
    fn new(folder: &String) -> Result<Todo, std::io::Error> {
        let my_home = string_to_static_str(folder.to_string());

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(format!("{}/.db.json", my_home))?;

        // Serializar el archivo json como HashMap
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("Ha ocurrido un error: {} 😱", e),
        }

        // let mut content = String::new();

        // f.read_to_string(&mut content)?;

        // // Uso de "turbofish": https://techblog.tonsser.com/posts/what-is-rusts-turbofish
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();

        // Ok(Todo { map })
    }

    fn insert(&mut self, key: String) {
        // Insertamos un nuevo valor en nuestro mapa.
        // Por default, el value va a ser true.
        self.map.insert(key, true);
    }

    fn save(self, folder: &String) -> Result<(), Box<dyn std::error::Error>> {
        let my_home = string_to_static_str(folder.to_string());
        // let mut content = String::new();

        // for (k, v) in self.map {
        //     let record = format!("{}\t{}\n", k, v);
        //     content.push_str(&record)
        // }

        // std::fs::write("/home/{user_home}/.db.json", content)

        // Abrir db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}/.db.json", my_home))?;

        // Escribir en el archivo con serde
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

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/*
 * https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/
 * https://doc.rust-lang.org/nightly/core/result/index.html
 * https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.collect
 * https://doc.rust-lang.org/reference/comments.html
 * https://doc.rust-lang.org/std/str/struct.SplitN.html
 * https://techblog.tonsser.com/posts/what-is-rusts-turbofish
 * https://www.koderhq.com/tutorial/rust/conditional-control/
 * https://www.koderhq.com/tutorial/rust/conditional-control/
 * https://www.rust-lang.org/es/what/cli
 * https://doc.rust-lang.org/nightly/std/boxed/index.html
 * https://stackoverflow.com/questions/6329887/compiling-problems-cannot-find-crt1-o
 */
