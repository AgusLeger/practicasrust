use std::io::stdin;

fn main()
{
    let buleano = false;
    let mut txt = String::new();

    stdin().read_line(&mut txt).expect("Error tipo1");

    let mut maralrra = txt.trim().parse::<bool>().expect("Error Parse") || buleano;

    println!("Binomo ||: {}",maralrra);

    maralrra = txt.trim().parse::<bool>().expect("Error Parse 2") && buleano;

    println!("Binomo &&: {}",maralrra);
}
/* 3- Escribir un programa que defina una variable de tipo booleano, y luego permita al usuario 
ingresar un valor booleano por teclado para actualizar su valor haciendo las operaciones 
and y or. Se deben imprimir ambos resultados. */