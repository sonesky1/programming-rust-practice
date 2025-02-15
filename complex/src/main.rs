use num::Complex;

///the Option<usize> is return value. Option in Rust' standard library is:
///  enum Option<T>   //has 2 return values in standard library
///   {
///    None,     //no t type is available
///    Some(T),  //some value where T is of type T
///   }


///to determine if c is in the mandelbrot set, using at most 'limit' iterati///ons to decice
fn escape_time(c: Complex<f64>, limit: usize) -> Option< usize> {
	 let mut z = Complex{ re: 0.0, im:0.0};
         for i in 0..limit {

            if z.norm_sqr() > 4.0 {
              return Some(i);
             }
             z = z * z + c;
           }
          
          None
         }


fn main() {
    let i = -0.0004;
    let m = 1000;
    //I think the i.into() makes it into the correct type? like casting?
    let m = escape_time(i.into(),m);
    println!("Escape time {:?}" , m);
}
