fn main()
{
    let arreglo : [i32; 5] = [3;5];
    println!("Cantidad de números impares: {}", cantidad_impares(arreglo));
}
pub fn cantidad_impares(arreglo: [i32;5]) -> i32 
{
    let mut contador = 0;
    for numero in arreglo
    {
        if numero % 2 != 0
        {
            contador += 1;
        }
    }
    contador
}
// 4- Definir la función llamada cantidad_impares que recibe como parámetro un arreglo de 
// números enteros y retorna la cantidad de números impares.