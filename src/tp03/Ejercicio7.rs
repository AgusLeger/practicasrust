#[derive(Clone, Debug)]
pub enum Color
{
    ROJO,
    VERDE,
    AZUL,
    AMARILLO,
    BLANCO,
    NEGRO
}
#[derive(Clone, Debug)]

pub struct Auto
{
    pub marca : String,
    pub modelo : String,
    pub anio : u32,
    pub precio_bruto : f32,
    pub color : Color
}
#[derive(Clone, Debug)]

pub struct ConcesionarioAuto
{
    pub nombre: String,
    pub direccion : String,
    pub cap_max : u32,
    pub autos : Vec<Auto>
}
impl ConcesionarioAuto
{
    pub fn new(nombre : String, direccion : String, cap_max : u32) -> ConcesionarioAuto
    {
        ConcesionarioAuto
        {
            nombre,
            direccion,
            cap_max,
            autos: Vec::new()
        }
    }
    pub fn agregar_auto(&mut self,auto : &Auto) -> bool
    {
        let agregado: bool = self.cap_max >= (self.autos.len() + 1) as u32;

        if agregado
        {
            self.autos.push(auto.clone());
        }
        return agregado
    }
    pub fn eliminar_auto(&mut self, auto : &Auto)
    {
        for i in 0..self.autos.len()
        {
            if self.autos[i].comparar_autos(auto)
            {
                self.autos.swap_remove(i);
                return
            }
        }
    }
    pub fn buscar_auto(&self, auto : &Auto) -> Option<Auto>
    {
        //let mut encontrado : Option<Auto> = None;
        for vehiculo in &self.autos
        {
            if vehiculo.comparar_autos(&auto)
            {
                return Some(vehiculo.clone())
            }
        }
        return None
    }
}
impl Auto
{
    pub fn new(marca : String, modelo : String, anio : u32, precio_bruto : f32, color : Color) -> Auto
    {
        Auto
        {
            marca,
            modelo,
            anio,
            precio_bruto,
            color
        }
    }
    pub fn color_string(&self) -> String
    {
        match self.color
        {
            Color::ROJO => String::from("ROJO"),
            Color::VERDE => String::from("VERDE"),
            Color::AZUL => String::from("AZUL"),
            Color::AMARILLO => String::from("AMARILLO"),
            Color::BLANCO => String::from("BLANCO"),
            Color::NEGRO => String::from("NEGRO"),
        }
    }
    pub fn comparar_autos(&self, auto : &Auto) -> bool
    {
        return (self.marca == auto.marca) && (self.modelo == auto.modelo) && (self.anio == auto.anio) && 
                (self.precio_bruto == auto.precio_bruto) && (self.color_string() == auto.color_string())
    }
    pub fn calcular_precio(auto : &Auto) -> f32
    {
        let mut variacion: f32;
        match auto.color
        {
            Color::ROJO | Color::AZUL | Color::AMARILLO => variacion = 0.25,
            _ => variacion = -0.10
        }
        if auto.marca == "BMW"
        {
            variacion = variacion + 0.15;
        }
        if auto.anio < 2000
        {
            variacion = variacion - 0.05;
        }

        if variacion > 0.0
        {
            auto.precio_bruto + (auto.precio_bruto * variacion)
        }else 
        {   //si queda variacion negativo el productoes negativo y por regla de signos se resta
            auto.precio_bruto + (auto.precio_bruto * variacion)
        }
    }
}

/*7- Defina una estructura llamada ConcesionarioAuto donde se conoce el nombre, la 
dirección y tiene una capacidad máxima para albergar X cantidad de autos. De los autos se 
conocen los campos de la marca, modelo, año, precio bruto y  color que pueden ser:rojo, 
verde, azul, amarillo, blanco o negro.
Para dichas estructuras implemente los siguientes métodos:
❖     ConcesionarioAuto:
➢ new: que pasando los parámetros correspondientes, crea un
ConcesionarioAuto y lo retorna.
➢ agregar_auto(auto): agrega un auto a la lista de autos que tiene sin superar
la máxima cantidad para albergarlos y retorna true, en caso de que lo supere
no lo agrega y retorna false.
➢ eliminar_auto(auto): elimina un auto de la lista de autos.
➢ buscar_auto(auto): busca un auto y si lo encuentra lo retorna.
❖     Auto:
➢ new: que pasando los parámetros correspondientes, crea un Auto y lo 
retorna. 
➢ calcular_precio: retorna el precio del auto aplicando los siguientes criterios:
■ si es de color primario le aplica un recargo del 25%, sino le aplica un
descuento del 10%. 
■ si la marca es BMW le aplica un recargo del 15%- 
■ si el año es menor a 2000 le aplica un descuento del 5%. */