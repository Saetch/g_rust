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
        let xdiff_start = self.start_punkt.0 - pos.0;
        let ydiff_start = self.start_punkt.1 - pos.1;
        let xdiff_end = self.end_punkt.0 - pos.0;
        let ydiff_end = self.end_punkt.1 - pos.1;
        //1. check wether or not the point is closer to the actual line or closer to the end-points!
        let distance_to_start = (xdiff_start.powi(2) + ydiff_start.powi(2)).sqrt();
        let distance_from_end  = (xdiff_end.powi(2) + ydiff_end.powi(2)).sqrt();
        if distance_to_start > distance_from_end + self.length || distance_from_end > distance_to_start + self.length{      //this means, that the point cannot be reached by a 90degree angle from the line, --> it is above the start or below the end..
            if distance_to_start < distance_from_end{
                return (distance_to_start, false);
            }else{
                return (distance_from_end, false);
            }
        }
        //2. if the point is closer to the line, get the closest distance
        return ( self.distance_to_right_angle_point(pos), true);
    }

    fn distance_to_right_angle_point(&self, pos :(f64, f64)) -> f64{
            //to get this, you need the distance between the pos (the point) and the crossing point between self and a helper vector, that looks like pos + h* self.normalvektor
            //in order to make this easier, we will be moving the original point to 0/0 and thus move the imaginary this (the mirror) the same way. This way we have less variables to worry about.

            // 0  +   normalvektor.0  =  (startpunkt.0 -pos.0) + ( linienvektor.0 )* m
            // 0  +   normalvektor.1  =  (startpunkt.1 -pos.1) + ( linienvektor.1 )* m

            let normalvektor = self.normalvektor.unwrap();
            let neuer_startpunkt = (self.start_punkt.0 - pos.0 , self.start_punkt.1 - pos.1);
            let spiegel_vector = self.linien_vektor;
            
                    //first row
        let mut first_non_m_variable = neuer_startpunkt.0;
        let mut first_m_multiplicant = spiegel_vector.0;    //this variable *m

        //second row
        let mut second_non_m_variable = neuer_startpunkt.1;
        let mut second_m_multiplicant = spiegel_vector.1;   //this variable *m

        //we know that the left side of the equasions look like:
        //first ::   normalvektor.0    * h
        //second::   normalvektor.1    * h

        //so to solve it, we divide both lines through their equivalent normalvektor. this way, on the left sides there will be only  1*h, for both equation
        if normalvektor.0 != 0.0{
            first_non_m_variable /= normalvektor.0;
            first_m_multiplicant /= normalvektor.0;
        }else{
            first_non_m_variable = 0.0;
            first_m_multiplicant = 0.0;
        }

        if normalvektor.1 != 0.0{
            second_m_multiplicant/= normalvektor.1;
            second_non_m_variable/= normalvektor.1;
        }
        else{
            second_non_m_variable = 0.0;
            second_m_multiplicant = 0.0;
        }


        //now we can subtract the first line from the second and this will result in an equation that looks like this: 
        // 0 = non_m_var + multiplicant * m

        let non_m_var = first_non_m_variable - second_non_m_variable;
        let multiplicant = first_m_multiplicant - second_m_multiplicant;

        //now we subtract non_m_var from 0, to get
        // (-1) * non_m_var = multiplicant*m

        let mut left_side =  -1.0* non_m_var;

        //next, we divide the left side through multiplicant to receive:
        // left_side/multiplicant = m
        // and we get m
        if multiplicant != 0.0{
            left_side /= multiplicant;    //<-- this is equal to m and we can put m in our mirror vector to receive the point the two vectors meet
        }else{
            left_side = 0.0;
        }


        let neuer_schnittpunkt = (first_non_m_variable + first_m_multiplicant* left_side , second_non_m_variable + second_m_multiplicant * left_side );

        return ( neuer_schnittpunkt.0.powi(2) + neuer_schnittpunkt.1.powi(2)).sqrt();
        
        }

}