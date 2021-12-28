pub struct Gerade{
    pub start_punkt: (f64, f64),
    pub linien_vektor: (f64, f64),
    pub end_punkt: (f64, f64),
}


impl Gerade{

    pub fn from_point_vec(punkt : (f64, f64), vektor: (f64, f64), laenge: f64) ->Self{
        let end = (punkt.0 + vektor.0*laenge, punkt.1+ vektor.1*laenge);
        Gerade{
            start_punkt: punkt,
            linien_vektor: vektor,
            end_punkt: end,
        }
    }

    pub fn from_two_points(startpunkt: (f64, f64), endpunkt: (f64, f64))-> Self{
        let vektor = (endpunkt.0 - startpunkt.0, endpunkt.1 - startpunkt.1);
        Gerade{
            start_punkt: startpunkt,
            linien_vektor: vektor,
            end_punkt: endpunkt,
        }
    }
/**
 * this function is supposed to reduce the vektor length to 1,
 * 
 */
    pub fn normalize(&mut self){

    }

}