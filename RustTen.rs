use std::io;
use std::io::Write;

fn main()
{
    println!("Escribe tu nombre: ");
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("No leyo la linea");

    println!("Escribiste el nombre {}", x);

    let v1 = std::io::stdout().write("Usted ".as_bytes()).unwrap();
    let v2 = std::io::stdout().write(String::from("es duro ").as_bytes()).unwrap();
    std::io::stdout().write(format!("\n{} cantidad de bytes escritos",(v1 + v2)).as_bytes()).unwrap();
}