pub struct Rectangulo
{
    pub longitud : u32,
    pub ancho : u32
}
impl Rectangulo
{
    pub fn new(longitud : u32, ancho : u32) -> Rectangulo
    {
        Rectangulo
        {
            longitud,
            ancho
        }
    }
    pub fn calcular_area(&self) -> u32
    {
        self.longitud * self.ancho
    }
    pub fn calcular_perimetro(&self) -> u32
    {
        (self.longitud + self.ancho)*2
    }
    pub fn es_cuadrado(&self) -> bool
    {
        self.longitud == self.ancho
    }
}


/*2- Escribir un programa que defina la estructura Rectangulo que tenga campos para la 
longitud y el ancho. Para dicha estructura implemente los siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea un Rectangulo y lo 
retorna. 
➢ calcular_area: calcular el área y la retorna. 
➢ calcular_perimetro: calcula el perímetro y lo retorna. 
➢ es_cuadrado: retorna true si es cuadrado, false caso contrario.*/