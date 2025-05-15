fn main()
{
    let array : [u32; 3] = [16,4,12];
    
    println!("LA SUMA DIO {}",suma_pares(array));
}
pub fn suma_pares(array : [u32;3]) -> u32
{
    let mut total = 0;
    for numero in array
    {
        if numero % 2 == 0
        {
            total += numero;
        }
    }
    total
}
/*3- Definir la función llamada suma_pares que recibe como parámetro un arreglo de 
números enteros y retorna la suma de los números pares. */