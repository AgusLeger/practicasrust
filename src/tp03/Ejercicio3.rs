use chrono::{DateTime, Days, Duration, Month, NaiveDate, TimeDelta, Datelike, Timelike, Utc};
#[derive(Clone, Debug)]
pub struct Fecha
{
    pub dia : u32,
    pub mes : u32,
    pub anio : u32 //con los naivetime se manejan es i32
}
impl Fecha
{
    pub fn new(dia :u32, mes :u32, anio :u32) -> Fecha
    {
        Fecha 
        {
            dia, 
            mes,
            anio
        }
    }                                                                           //(YEAR,MONTH,DAY)(DEVUELVE UN OPTION)SI LA FECHA ES VALIDA
    pub fn es_fecha_valida(&self) -> bool           //USANDO NAIVE POR CLASE, EL FROM_YMD_OPT y el .is_some() DEVUELVE UN TRUE SI ES ALGO Y UN FALSE SI DEVUELVE UN NONE 
    {
        NaiveDate::from_ymd_opt(self.anio as i32, self.mes, self.dia).is_some()
    }
    /*
    pub fn es_fecha_valida(&self) -> bool                      //quizas un Match
    {
        let mut validez: bool = false;
        if (self.mes >= 1 && self.dia >= 1) && (self.mes <= 12 && self.dia <= 31 && self.anio > 0)
        {   
            if self.mes != 1 && self.mes != 3 && self.mes != 5 && self.mes != 7 && self.mes != 8 && self.mes != 10 && self.mes != 12
            {
                if (self.mes != 4 || self.mes != 6 || self.mes != 9 || self.mes != 11) || self.dia <= 30  //ANDA BIEN LISTO, RE TESTEADO
                {
                    if self.es_bisiesto()
                    {
                        if self.dia <= 29
                        {
                            validez = true;
                        }
                    }
                    else if self.dia <= 28
                    {
                        validez = true;
                    }
                }
            }
            else
            {
                validez = true;
            }
        }
        validez
    }*/
    pub fn es_bisiesto(&self) -> bool       //Puede ingresar un 0, ¿panic?
    {
        (self.anio % 4 == 0 && self.anio % 100 != 0) || (self.anio % 400 == 0)
    }
    pub fn sumar_dias(&mut self, dias : u32)
    {                                           //naivedate es un from_ymd y le paso la fecha 
        let mut fecha: NaiveDate = NaiveDate::from_ymd(self.anio as i32, self.mes as u32, self.dia as u32);
        fecha = fecha + Duration::days(dias as i64);
        self.dia = fecha.day() as u32;
        self.mes = fecha.month() as u32;
        self.anio = fecha.year() as u32;
    }
    pub fn restar_dias(&mut self, dias : u32)
    {
        let mut fecha: NaiveDate = NaiveDate::from_ymd(self.anio as i32, self.mes as u32, self.dia as u32);
        fecha = fecha - Duration::days(dias as i64);
        self.dia = fecha.day() as u32;
        self.mes = fecha.month() as u32;
        self.anio = fecha.year() as u32;
    }
    pub fn es_mayor(&self, otra_fecha : &Fecha) -> bool
    {
        let mifecha : NaiveDate = NaiveDate::from_ymd(self.anio as i32, self.mes, self.dia);
        let laotra : NaiveDate = NaiveDate::from_ymd(otra_fecha.anio as i32, otra_fecha.mes, otra_fecha.dia);

        mifecha > laotra
    }
    pub fn comparar_fecha(&self, otra_fecha : &Fecha) -> bool
    {
        otra_fecha.dia == self.dia && otra_fecha.mes == self.mes && otra_fecha.anio == self.anio
    }
} 
/*3- Escribir un programa que defina una estructura Fecha que tenga campos para el día, el 
mes y el año. Para dicha estructura implemente los siguientes métodos: 
➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna. 
➢ es_fecha_valida: retorna true si es una fecha válida, false caso contrario.//tenga en 
cuenta los años bisiestos también. 
➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto. 
➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose 
➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose 
➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
la fecha pasada por parámetro. */