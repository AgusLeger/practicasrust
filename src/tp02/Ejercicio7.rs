fn main()
{
    let arreglo : [i32;5] = [-150,200,12,51,98];
    
    println!("Cantidad de números mayores: {}",cantidad_de_mayores(arreglo, 60));
}
fn cantidad_de_mayores(array : [i32;5], limite : i32) -> u32
{
    let mut contador : u32 = 0;
    
    for elemento in array
    {
        if limite > elemento
        {
            contador += 1;
        }
    }
    contador
}
/*7-Definir la función llamada cantidad_de_mayores que recibe como parámetro un arreglo 
de números enteros y un número entero llamado límite. Esta función retorna la cantidad de 
números mayores al límite que tiene el arreglo.  */