use domain::entities::Todo;
use std::io;
use std::io::Write;

mod domain;

fn menu() -> u8 {
    println!("\nMENU");
    println!("========================================\n");
    println!("Por favor, selecciona una opción:");
    println!("\t 1. Mostrar la lista de todos");
    println!("\t 2. Añade un todo");
    println!("\t 3. Completa un todo por su nombre");
    println!("\t 4. Elimina un todo por su nombre");
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

fn create_menu(user_home: &str) {
    while {
        let mut todo = Todo::new(user_home).expect("La inicialización de la db falló 😱");

        let option: u8 = menu();

        match option {
            1 => {
                println!("\nLISTA DE TODOS");
                println!("========================================\n");

                let vec = todo.map.iter();

                for (i, entry) in vec.enumerate() {
                    let completed = if (entry.1).0 { "activo" } else { "inactivo" };
                    println!(
                        "\t {}. {} => {} / 🕒 {}",
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

                todo.insert(&item);

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
                    None => println!("'{}' no está presente en la lista de ToDos", item.trim()),
                    Some(_) => match todo.save(&user_home) {
                        Ok(_) => println!("Todo actualizado"),
                        Err(why) => println!("Ha ocurrido un error: {} 😱", why),
                    },
                }
            }

            4 => {
                println!("\nPor favor, ingresa el nombre del todo:");
                let mut item: String = String::new();

                io::stdin()
                    .read_line(&mut item)
                    .expect("Fallo al leer la línea!");

                match todo.delete(&item) {
                    None => println!("'{}' no está presente en la lista de ToDos", item.trim()),
                    Some(entry) => match todo.save(&user_home) {
                        Ok(_) => println!("El todo '{}' ha sido eliminado", entry),
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

fn get_user_home() -> String {
    let mut user_home = String::new();

    match home::home_dir() {
        Some(path) => match path.join(".db.json").to_str() {
            None => println!("¡El path no es una secuencia UTF-8 válida! 😱"),
            Some(my_home) => user_home = my_home.to_string(),
        },
        None => println!("¡Imposible conseguir el directorio Home! 😱"),
    }

    user_home
}

fn main() {
    create_menu(&get_user_home());
}

/*
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
// VER NOTA-1 ABAJO:
*/

/*
 * NOTA-1
 * How to Convert a String into a &'static Str in Rust:
 * https://installmd.com/c/154/rust/convert-a-string-into-a-static-str
 * https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
 */

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
