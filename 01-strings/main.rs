

/*
&str: These are called ‘string slices’. 
A string slice has a fixed size, and cannot be mutated. 
It is a reference to a sequence of UTF-8 bytes.
*/


fn main() {
  let name = "Bob";
  println!("{}", name);
  let mut s = "Hello, world!".to_string();
  s = s + " 🌍";
  println!("{}",&s);
} 