pub struct Producto
{
    pub nombre : String,
    pub precio_bruto : f32,
    pub numero : u32
}
impl Producto  
{
    pub fn new(nombre : String, precio_bruto : f32, numero : u32) -> Producto
    {
        Producto 
        { 
            nombre, 
            precio_bruto, 
            numero
        }
    }
    pub fn calcular_impuestos(&self ,impuestos : f32) -> f32
    {
        return   (self.precio_bruto * impuestos) / 100.0
    }
    pub fn aplicar_descuento(&self, descuento : f32) -> f32
    {
        return   (self.precio_bruto * descuento) / 100.0
    }
    pub fn calcular_precio_total(&self, impuestos : Option<f32>, descuento : Option<f32>) -> f32
    {
        let mut precio_bruto: f32 = self.precio_bruto;
        if let Some(descu) = descuento
        {
            precio_bruto = precio_bruto - self.aplicar_descuento(descu);
        }
        if let Some(robo) = impuestos
        {
            precio_bruto = precio_bruto + self.calcular_impuestos(robo);
        }
        precio_bruto
    }
}


/*5- Escribir un programa que defina una estructura Producto que tenga campos para el 
nombre, el precio bruto y un número identificatorio. Para dicha estructura implemente los 
siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea un Producto y lo retorna. 
➢ calcular_impuestos(porcentaje_de_impuestos): retorna el valor de impuestos sobre 
el precio bruto 
➢ aplicar_descuento(porcentaje_de_descuento): retorna el valor del porcentaje de 
descuento sobre el precio bruto 
➢ calcular_precio_total(porcentaje_de_impuestos, porcentaje_descuento): retorna el 
precio total a pagar aplicando impuesto y descuento. Tenga en cuenta que los 
parámetros son opcionales.  */