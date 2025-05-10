use std::io::stdin;

fn main()
{
    let testo = "Maralrrá ".to_string();
    let mut txt_usuario = String::new();

    stdin().read_line(&mut txt_usuario).expect("ERROR AL LEER");

    //testo = testo + &txt_usuario;
    //testo.push_str(&txt_usuario);
    txt_usuario = testo + &txt_usuario;
    println!("{}",txt_usuario.to_uppercase());
}
/*5- Escribir un programa que defina una variable de tipo cadena, y luego permita al usuario 
ingresar una cadena por teclado para concatenar su valor. El programa debe imprimir la 
cadena en mayúsculas.*/