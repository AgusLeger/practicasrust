 #[derive(Debug)]
pub struct Persona 
{
    pub nombre:String,
    pub edad :u8,
    pub direccion : Option<String>
}
impl Persona
{
    pub fn new(nombre: String, edad : u8, direccion : Option<String>) -> Persona
    {
        Persona
        {  
            nombre,
            edad,
            direccion
        }
    }
    pub fn to_string(&self) -> String
    {
        format!("{:?}",self)
    }
    pub fn obtener_edad(&self) -> u8
    {
        self.edad
    }
    pub fn actualizar_direccion(&mut self, dir : String)
    {
        self.direccion = Some(dir);
    }
}
/*1- Escribir un programa que defina una estructura Persona que tenga campos para el 
nombre, la edad y la dirección(que puede ser nulo al momento de la creación de una 
persona). Para dicha estructura implemente los siguientes métodos: 
➢  new: que pasando los parámetros correspondientes, crea una Persona y la retorna. 
➢  to_string: que retorna un string con  los datos de la persona concatenados sobre el 
mensaje ejecutado por ej: 
person.to_string() , donde person es una variable del tipo Persona. 
➢  obtener_edad: retorna la edad de la persona. 
➢  actualizar_direccion(nueva_direccion)  */