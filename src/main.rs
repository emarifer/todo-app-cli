use chrono::{Local, Timelike, Utc};
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn menu() -> u8 {
    println!("\nMENU");
    println!("----------------------------------------\n");
    println!("Por favor, selecciona una opción:");
    println!("\t 1. Mostrar la lista de todos");
    println!("\t 2. Añade un todo");
    println!("\t 3. Completa un todo por su nombre");
    println!("\t 0. Salir de la aplicación");
    print!("\nOpción: ");
    io::stdout().flush().expect("Error flushing");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("No se pudo leer la línea 😱");

    let option = input.trim().parse();

    match option {
        Ok(o) => o,
        Err(_) => {
            eprintln!("ERROR: Por favor, escribe un número");
            99
        }
    }
}

fn main() {
    let mut user_home = String::new();

    match home::home_dir() {
        Some(path) => match path.to_str() {
            None => println!("¡Imposible conseguir el directorio Home! 😱"),
            Some(my_home) => user_home = my_home.to_string(),
        },
        None => println!("¡Imposible conseguir el directorio Home! 😱"),
    }

    while {
        let mut todo = Todo::new(&user_home).expect("La inicialización de la db falló 😱");

        let option: u8 = menu();

        match option {
            1 => {
                println!("\nLISTA DE TODOS");
                println!("----------------------------------------\n");

                let vec = todo.map.iter();

                for (i, entry) in vec.enumerate() {
                    let completed = if (entry.1).0 { "activo" } else { "inactivo" };
                    println!(
                        "\t {}. {} => {} / [{}]",
                        i + 1,
                        entry.0,
                        completed,
                        (entry.1).1,
                    );
                }
            }

            2 => {
                println!("\nPor favor, ingresa el nombre del todo:");
                let mut item: String = String::new();

                io::stdin()
                    .read_line(&mut item)
                    .expect("Fallo al leer la línea!");

                todo.insert(item);

                match todo.save(&user_home) {
                    Ok(_) => println!("Todo guardado correctamente 😀"),
                    Err(why) => println!("Ha ocurrido un error: {} 😱", why),
                };
            }

            3 => {
                println!("\nPor favor, ingresa el nombre del todo:");
                let mut item: String = String::new();

                io::stdin()
                    .read_line(&mut item)
                    .expect("Fallo al leer la línea!");

                match todo.complete(&item) {
                    None => println!("'{}' no está presente en la lista de ToDos", item),
                    Some(_) => match todo.save(&user_home) {
                        Ok(_) => println!("Todo actualizado"),
                        Err(why) => println!("Ha ocurrido un error: {} 😱", why),
                    },
                }
            }

            0 => println!("Saliendo..."),
            99 => {}
            _ => println!("Opción inválida"),
        };

        option != 0
    } {}
}

struct Todo {
    // Usamos el tipo HashMap que está incorporado en Rust.
    map: HashMap<String, (bool, String)>,
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
    }

    fn insert(&mut self, key: String) {
        // Insertamos un nuevo valor en nuestro mapa.
        // Por default, el value va a ser true.
        self.map
            .insert(key.trim().to_string(), (true, get_time_and_data()));
    }

    fn save(self, folder: &String) -> Result<(), Box<dyn std::error::Error>> {
        let my_home = string_to_static_str(folder.to_string());

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
        match self.map.get_mut(key.trim()) {
            Some(v) => Some(*v = (false, get_time_and_data())),
            None => None,
        }
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn get_time_and_data() -> String {
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

/*
 * https://crates.io/crates/home
 * https://crates.io/crates/chrono
 * https://rustrepo.com/repo/chronotope-chrono-rust-date-and-time
 * https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html#method.parse_from_rfc3339
 * https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
 * https://doc.rust-lang.org/std/primitive.usize.html
 * https://stackoverflow.com/questions/66288515/how-do-i-get-the-index-of-the-current-element-in-a-for-loop-in-rust
 * https://stackoverflow.com/questions/44788026/expected-type-bool-found-type-bool
 * https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
 * https://stackoverflow.com/questions/21324657/does-rust-support-ruby-like-string-interpolation
 * https://stackoverflow.com/questions/68608378/error-says-value-moved-in-previous-in-iteration-of-loop-but-this-doesnt-seem-t
 * https://doc.rust-lang.org/reference/expressions/tuple-expr.html
 */

/*
 * https://code.visualstudio.com/docs/supporting/faq#_resolving-shell-environment-fails
 */
