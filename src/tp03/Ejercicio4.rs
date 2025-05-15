#[derive(Debug)]
//#[derive(PartialEq)]
pub enum Tipo
{
    Equilatero,
    Isosceles,
    Escaleno
}
pub struct Triangulo
{
    pub lados : (f32, f32, f32),
    pub tipo : Option<Tipo>
}
impl Triangulo
{
    pub fn new(lado0: f32, lado1: f32, lado2 : f32) -> Triangulo
    {
        let lados: (f32, f32, f32) = (lado0,lado1,lado2);
        let tipo : Option<Tipo> = None;
        let mut aux: Triangulo = Triangulo
        {
            lados,
            tipo
        };
        aux.determinar_tipo();
        aux
    }
    pub fn determinar_tipo(&mut self)
    {
        if  (self.lados.0 + self.lados.1 + self.lados.2) == (self.lados.0 * 3.0)
        {
            self.tipo = Some(Tipo::Equilatero);
        }
        else if (self.lados.0 == self.lados.1) || (self.lados.1 == self.lados.2) || (self.lados.0 == self.lados.2)   
        {
            self.tipo = Some(Tipo::Isosceles);
        }
        else 
        {
            self.tipo = Some(Tipo::Escaleno);
        }
    }
    pub fn calcular_area(&self) -> f32
    {
        //semi_perimetro
        let s: f32 =    (self.lados.0 + self.lados.1 + self.lados.2) / 2.0;
        let area : f32 = (s*(s - self.lados.0)*(s - self.lados.1)*(s - self.lados.2)).powf(0.5); //fórmula mientras se sepan todos los lados 🤷‍♂️
        area
    }
    pub fn calcular_perimetro(&self) -> f32
    {
        self.lados.0 + self.lados.1 + self.lados.2
    }
}
/*4- Escribir un programa que defina la estructura Triangulo que tenga campos para las 
longitudes de sus tres lados. Para dicha estructura implemente los siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea un Triangulo y lo retorna. 
➢ determinar_tipo: retorna el tipo del Triangulo, los tipos pueden ser equilatero, 
isósceles o escaleno. 
➢ calcular_area: calcular el área y la retorna. 
➢ calcular_perimetro: calcula el perímetro y lo retorna.*/