pub struct Gerade{
    pub start_punkt: (f64, f64),
    pub linien_vektor: (f64, f64),
    pub end_punkt: (f64, f64),
    pub normalvektor: Option<(f64, f64)>,       //Option can be None (equivalent to Null in other languages)
    pub closest_point_to_zerozero: Option<(f64, f64)>
}


impl Gerade{

    pub fn from_point_vec(punkt : (f64, f64), vektor: (f64, f64), laenge: f64) ->Self{
        let end = (punkt.0 + vektor.0*laenge, punkt.1+ vektor.1*laenge);
        Gerade{
            start_punkt: punkt,
            linien_vektor: vektor,
            end_punkt: end,
            normalvektor: None,
            closest_point_to_zerozero: None,
        }
    }

    pub fn from_two_points(startpunkt: (f64, f64), endpunkt: (f64, f64))-> Self{
        let vektor = (endpunkt.0 - startpunkt.0, endpunkt.1 - startpunkt.1);
        Gerade{
            start_punkt: startpunkt,
            linien_vektor: vektor,
            end_punkt: endpunkt,
            normalvektor: None,
            closest_point_to_zerozero: None,
        }
    }

    pub fn has_normal(&self)-> bool{
        self.normalvektor.is_some()
    }
/**
 * this function is supposed to reduce the vektor length to 1,
 * 
 */
    pub fn normalize(&mut self){
        let veklength = (self.linien_vektor.0*self.linien_vektor.0 + self.linien_vektor.1*self.linien_vektor.1).sqrt();

        self.linien_vektor.0 = self.linien_vektor.0 / veklength;
        self.linien_vektor.1 = self.linien_vektor.1 / veklength;
        if !self.has_normal(){
            self.calculate_normal_vektor();
        }
    }


    pub fn calculate_normal_vektor(&mut self){
        let vektor = (-self.linien_vektor.1, self.linien_vektor.0);
        self.normalvektor = Some(vektor);
    }

    pub fn only_normalize(&mut self){
        
        let veklength = (self.linien_vektor.0*self.linien_vektor.0 + self.linien_vektor.1*self.linien_vektor.1).sqrt();

        self.linien_vektor.0 = self.linien_vektor.0 / veklength;
        self.linien_vektor.1 = self.linien_vektor.1 / veklength;
    }

}