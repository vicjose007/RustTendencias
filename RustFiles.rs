use std::fs::File;
use std::io::prelude::*;

fn main()
{
    let mut file = File::create("archivo.txt"). expect("No se pudo crear archivo");

    file.write_all(b"Saludos profesor").expect("No se pudo ingresar al archivo");

    let mut file = File::open("archivo.txt").expect("No se abre el archivo");

    let mut contenido = String::new();

    file.read_to_string(&mut contenido).expect("No se pudo leer archivo");

    println!("Contenido del archivo: \n {}", contenido);
}

