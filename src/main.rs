use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    jugar();
}

fn solicitar_numero_usuario() -> String {
    // Solicita el numero al usuario

    let mut numero = String::new(); // Crea una variable mutable para almacenar el numero del usuario

    println!("INGRESA UN NUMERO ENTRE 1 Y 10:"); // Imprime un mensaje para solicitar el numero

    io::stdin() // Lee la entrada del usuario
        .read_line(&mut numero) // Lee la entrada del usuario
        .expect("Failed to read line"); // Maneja el error
    numero // Retorna el numero del usuario
}

fn imprimir_numero(mensaje: &str, numero_usuario: u32) {
    // Imprime el numero del usuario
    println!("{} {}", mensaje, numero_usuario); // Imprime el numero del usuario
}

fn transformar_numero(numero_usuario: String) -> Result<u32, std::num::ParseIntError> {
    // Transforma el String a u32
    numero_usuario
        .trim() // Elimina los espacios en blanco
        .parse() // Transforma el String a u32
}

fn generar_numero_aleatorio() -> u32 {
    let mut rng = rand::rng();
    let numero_aleatorio = rng.random_range(1..=10); // Genera un numero aleatorio
    numero_aleatorio // Retorna el numero aleatorio
}

fn comparar_numeros(numero_usuario: u32, numero_aleatorio: u32) -> bool {
    match numero_usuario.cmp(&numero_aleatorio) {
        Ordering::Less => {
            println!("Tu numero es menor");
            false
        }
        Ordering::Greater => {
            println!("Tu numero es mayor");
            false
        }
        Ordering::Equal => {
            println!("Tu numero es igual");
            true
        }
    }
}

fn jugar() {
    let mut intentos = 0;
    let max_intentos = 5;

    let numero_aleatorio = generar_numero_aleatorio(); // Genera un numero aleatorio
    imprimir_numero("EL NUMERO ALEATORIO ES:", numero_aleatorio); // Imprime el numero aleatorio

    loop {
        println!("GUESSING GAME"); // Imprime el nombre del juego
        let numero_usuario = transformar_numero(solicitar_numero_usuario());
        match numero_usuario {
            Ok(numero_usuario) => {
                intentos += 1;
                println!("Intento {}", intentos);
                println!("Intentos restantes: {}", max_intentos - intentos);
                if intentos == max_intentos {
                    println!("❌ Has perdido.");
                    break;
                }
                imprimir_numero("TU NUMERO ES:", numero_usuario);
                let resultado = comparar_numeros(numero_usuario, numero_aleatorio);
                match resultado {
                    true => {
                        println!("🎉 ¡FELICIDADES! HAS GANADO.");
                        break;
                    }
                    false => {
                        println!("❌ No es correcto.");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("⚠️ Eso no es un número válido. Intenta de nuevo.");
                continue;
            }
        }
    }
}
