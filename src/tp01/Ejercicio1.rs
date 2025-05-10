use std::io::stdin;

fn main ()
{
    let numero = 14;
    let mut texto ;

    stdin().read_line(&mut texto).expect("Error");

    let num_usuario : i32 = texto.trim().parse::<i32>().expect("Error");

    println!("entrada: {num_usuario}");
    println!("Resultado1 = {} \nResultado2 = {} \nResultado3 = {} \nResultado4 = {}",numero * num_usuario, numero / num_usuario, numero + num_usuario, numero - num_usuario);
    
}