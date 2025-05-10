use std::io::stdin;

fn main()
{
    let arreglo : [String; 5] = ["maralrra".to_string(), "fred".to_string(), "jamon".to_string(), "bazuca".to_string(), "impotencia".to_string()];
    let mut testo = String::new();

    stdin().read_line(&mut testo).expect("ERROR AL LEER");
    testo = testo.trim().to_string();

    for palabra in arreglo
    {
        if palabra == testo
        {
            println!("{palabra} ES IGUAL A {testo}");
        }
        //println!("{palabra}");
    }
}
/*11- Escribir un programa que defina un arreglo de 5 cadenas, y luego permita al usuario 
ingresar una cadena por teclado. El programa debe imprimir un mensaje si la cadena 
ingresada por el usuario se encuentra en el arreglo. */