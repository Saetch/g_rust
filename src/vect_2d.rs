use crate::gerade::Gerade;

pub struct Vector2D{
    pub x : f64,
    pub y : f64,
}


impl Vector2D {
    pub fn mirror_on(&mut self, gerade : &Gerade, speed: f64){

        if !gerade.has_normal(){
            println!("Vector2D->mirror_on called on non-normalized vector! Skipping");
            return;
        }

        //let the helper line go through 0/0 
        let _ursprung = (0.0, 0.0);
        let normalvektor = gerade.normalvektor.unwrap();

        //let the line to be mirrored on go through the end of this vector. This doesn't matter for the direction of the reflected Vector. The actual place is only relevant for the points if mirrored. The lines in math that we will be looking at, extend to infinity
        let schnittpunkt = (self.x,self.y);
        let spiegel_vector = gerade.linien_vektor;


        //now you need to find where the helper line and the mirror meet. The final resulting vector will be this point times 2


        // the vectors can be multiplied whatever number of times.  h times for the helper, m times for the mirror.
        //this will result in a equation system like:
        // helper: 0 / 0 + h* (2(x) / 1(y))                    //because we let the _ursprung be 0/0, these values don't need to be processed
        // mirror: 5(x) / 3(y) +  m * (1(x) / 3(y))

        //-->
        // 0 + 2h = 5+ 1m     <-- for x value (0)    ---   normalvektor.0 * h = first_non_m_variable + first_m_multiplicant * m
        // 0 + 1h = 3+ 3m     <-- for y value (1)    ---   normalvektor.1 * h = second_non_m_variable + second_m_multiplicant * m
        // we need to solve this system for one of the variables and put them into the correct equation to get the point.
        // this means, we can divide through h and then subtract the x /y values, first for the regular and then for the m
        
        //first row
        let mut first_non_m_variable = schnittpunkt.0;
        let mut first_m_multiplicant = spiegel_vector.0;    //this variable *m

        //second row
        let mut second_non_m_variable = schnittpunkt.1;
        let mut second_m_multiplicant = spiegel_vector.1;   //this variable *m

        //we know that the left side of the equasions look like:
        //first ::   normalvektor.0    * h
        //second::   normalvektor.1    * h

        //so to solve it, we divide both lines through their equivalent normalvektor. this way, on the left sides there will be only  1*h, for both equation

        first_non_m_variable /= normalvektor.0;
        first_m_multiplicant /= normalvektor.0;

        second_m_multiplicant/= normalvektor.1;
        second_non_m_variable/= normalvektor.1;

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

        left_side /= multiplicant;    //<-- this is equal to m and we can put m in our mirror vector to receive the point the two vectors meet


        //this is the point, where the normalvektor through 0/0 (where our imaginary vector starts) goes throug and meets the (moved) mirrorline. meaning 0/0 -> crossing-point is half of 0/0 -> mirrored point
        let actual_crossing_point = (    first_non_m_variable + first_m_multiplicant* left_side /* left side == m */     ,  second_non_m_variable + second_m_multiplicant * left_side   );
        self.x = actual_crossing_point.0 * 2.0;
        self.y = actual_crossing_point.0 * 2.0;


        //now just make sure the vector still is the same length, so the speed does not get messed up
        let self_speed = (self.x*self.x + self.y * self.y).sqrt();
        let speed_multiplicant = speed / self_speed;
        self.x *= speed_multiplicant;
        self.y *= speed_multiplicant; 
    }
}