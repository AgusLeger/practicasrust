fn main()
{
    let arreglo : [i32;5] = [1,2,3,4,5];

    println!("{}",cantidad_en_rango(arreglo,3,5)); //3
}
fn cantidad_en_rango( ar : [i32;5], inferior : i32, superior : i32) -> u32
{
    let mut contador : u32 = 0;

    for elemento in ar
    {
        if elemento >= inferior && elemento <= superior
        {
            contador += 1;
        }
    }
    contador
}
/*9-Definir la función llamada cantidad_en_rango que recibe 3 parámetros: 1 arreglo de 
enteros, un número entero llamado inferior y otro número entero llamado superior. Esta 
función retorna la cantidad de números del arreglo que están entre el rango de los 
parámetros inferior y superior inclusive. */