use std::str::FromStr;
use std::env;

fn main() {
   let mut nums = Vec::new();

   for arg in env::args().skip(1){
      nums.push(u64::from_str(&arg).expect("error parsing argument")); //what is this?
    
   }
   if nums.len() == 0 {
     eprintln!("Usage: multiply ...");
     std::process::exit(1);
   }

   let mut d = nums[0];
   for m in &nums[1..] {
     d = multiply(d, *m);
  }

   
   
    println!("All the numbers multiplied {:?} is {}", nums, d);


}
//multiply 2 numbers
fn multiply(n: u64, m: u64) -> u64 {

   assert!(n != 0 && m != 0);

   let p = m * n;
   p
  


}

#[test]
fn test_multiply(){
    assert_eq!(multiply(6, 1), 6);

}
