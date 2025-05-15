fn main()
{
    let condenado : [String;5] = ["a".to_string(), "aa".to_string(),"aaa".to_string(),"aaaa".to_string(),"aaaaa".to_string()];

    println!("{}",cantidad_de_cadenas_mayor_a(condenado,2));
}
fn cantidad_de_cadenas_mayor_a(array : [String;5], limite : u32) -> u32
{
    let mut contador : u32;
    let mut contador_final : u32 = 0;

    for palabra in array //con esto recorro la cadena
    {
        contador = 0;
        for chars in palabra.chars() //recorro los caracteres de la palabra
        {
            contador += 1;              //habrá alguna función para ver el tamaño de los strings?
        }
        if contador > limite
        {
            contador_final += 1;
        }
    }
    contador_final

}
/*10-Definir la función llamada cantidad_de_cadenas_mayor_a que recibe como parámetros 
un arreglo de String y un entero llamado límite. Esta función retorna la cantidad de Strings 
del arreglo que son de longitud mayor al parámetro límite. */