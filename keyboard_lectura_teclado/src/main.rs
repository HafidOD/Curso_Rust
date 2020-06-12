// importar la libreria 
use std::io;

fn main() {
    println!("Ingresa tu nombre:");
    let mut username = String::new(); // metodo estatico, es decir, no se necesita instanciar, para usar el metodo
    // dicho metodo regresa un string vacio

    // read_line retorna 2 result, estructura que tiene 2 resultados exito o error
    io::stdin().read_line(&mut username); // prestamo por referencia se usa &, al igual que se presta mutablilidad &mut (permiso de lectura y escritura)
    // es decir read_line va a tener permiso de lectura y escritura de la variable que se le pase como argumento

    let username = username.trim(); // metodo trim elimina los saltos de linea al terminar la ejecucion, comentar para ver dif
    
    println!("Ingresa tu edad:");
    let mut edad = String::new();

    io::stdin().read_line(&mut edad);
    let edad = edad.trim();
    
    // de igual forma es una estrutura Result, exito o error
    let edad: i32 = edad.parse().unwrap(); //convertir string a int

    println!("Hola {}, tu edad es: {}", username, edad);
}
