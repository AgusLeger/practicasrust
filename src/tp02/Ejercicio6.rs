fn main ()
{
    let chain : [String;5] = ["a".to_string(),"ab".to_string(),"abc".to_string(),"abcd".to_string(),"".to_string()];
    let cantidad_char : [usize;5] = longitud_de_cadenas(&chain);

    println!("{:?} \n {:?}",chain, cantidad_char); //si pongo esto tengo que mandar un .clone() de chain por el owner
}
fn longitud_de_cadenas(cadena : &[String;5]) -> [usize;5]
{
    //let mut numero : u32;
    let mut cant : [usize;5] = [0;5];

    for i in 0..cadena.len()
    {
        //numero = 0;
        /*for chare in cadena[i].chars()
        {
            numero += 1;
        }*/
        cant[i] = cadena[i].len();
        //cant[i] = numero;
    }
    cant
}
/*6-Definir la función llamada longitud_de_cadenas que recibe un arreglo de String y retorna 
un arreglo con la longitud de las cadenas del parámetro, correspondiéndose en posición del 
arreglo. */