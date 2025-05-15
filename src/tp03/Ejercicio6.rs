#[derive(Clone, Debug)]
pub struct Examen
{
    pub materia : String,
    pub nota    : f32
}
#[derive(Clone, Debug)]

pub struct Estudiante
{
    pub nombre : String,
    pub id : u32,
    pub calif : Vec<Examen>
}

impl Examen
{
    pub fn new(materia : String, nota : f32) -> Examen
    {
        Examen
        {
            materia,
            nota
        }
    }
}
impl Estudiante
{
    pub fn new(nombre : String, id : u32) -> Estudiante
    {
        Estudiante
        {
            nombre,
            id,
            calif : Vec::new()
        }
    }
    pub fn obtener_promedio(&self) -> f32
    {  
        let mut suma : f32 = 0.0;

        for nota in &self.calif
        {
            suma = suma + nota.nota;
        }
        suma / self.calif.len() as f32
    }
    pub fn obtener_calificacion_mas_alta(&self) -> f32
    {
        let mut max = -1.0;
        for nota in &self.calif
        {
            if nota.nota > max
            {
                max = nota.nota;
            }
        }
        max
    }
    pub fn obtener_calificacion_mas_baja(&self) -> f32
    {
        let mut min = 9999.0;
        for nota in &self.calif
        {
            if nota.nota < min
            {
                min = nota.nota;
            }
        }
        min
    }
}

/*6- Escribir un programa que defina una estructura Estudiante que tenga campos para el 
nombre, el número de identificación y las calificaciones de exámenes. De cada Examen se 
conoce el nombre de la materia y la nota. Para dichas estructuras implemente los siguientes 
métodos: 
❖ Examen: 
➢ new: que pasando los parámetros correspondientes, crea un Examen y lo 
retorna. 
❖ Estudiante:
➢ new: que pasando los parámetros correspondientes, crea un Estudiante y lo 
retorna. 
➢ obtener_promedio: retorna el promedio de las notas.
➢ obtener_calificacion_mas_alta: retorna la nota más alta. 
➢ obtener_calificacion_mas_baja: retorna la nota más baja.*/