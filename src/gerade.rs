pub struct Gerade{
    pub start_punkt: (f64, f64),
    pub linien_vektor: (f64, f64),
    pub length: f64,
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
            length: laenge,
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
            length: ((startpunkt.0 - endpunkt.0).powi(2) + (startpunkt.1 - endpunkt.1).powi(2)).sqrt(),
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

    /**
     * returns the distance and wether or not the point can be reached by a 90 degrees turn at the line
     */
    pub fn distance_to(&self, pos: (f64, f64)) -> (f64, bool){
        //1. check wether or not the point is closer to the actual line or closer to the end-points!
        let distance_to_start = (self.start_punkt.0.powi(2) + self.start_punkt.1.powi(2)).sqrt();
        let distance_from_end  = (self.end_punkt.0.powi(2) + self.end_punkt.1.powi(2)).sqrt();
        if distance_to_start > distance_from_end + self.length || distance_from_end > distance_to_start + self.length{      //this means, that the point cannot be reached by a 90degree angle from the line, e.g. it is above the start or below the end..
            if distance_to_start < distance_from_end{
                return (distance_to_start, false);
            }else{
                return (distance_from_end, false);
            }
        }
        //2. if the point is closer to the line, get the closest distance
        return ( 0.0, true);
    }

}