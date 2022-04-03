//===============================================================
struct Template{ // this is used to create a custom data type -- its also comparable to a pyton dictionary
  x: u32,
  y: i32,
  flag: bool
}
//===============================================================

fn main(){
  let mut counter: i32 = 0;
  //say_hello("Sid");
  while counter <= 10{
    say_hello("Sid");
    counter+=1;
  }
  println!("---------------------------");
  sample_declarations();

  let day : &str = "Sunday";

  if day == "Monday"{
    println!("Monday blues!!");
  }
  else if day == "Sunday"{
    println!("I am gonna be very lazy today!");
  }
  else{
    println!("Keep hustling..");
  }

  times_two(20);
  let mut times_three_output : i32 = times_three(3);
  println!("The 'times three function' output is: {}",times_three_output);

  let temp = Template{
    x: 99,
    y: -7,
    flag: true
  };

  println!("The value of 'flag' within our struct 'Template' is {}", temp.flag);
  using_std_library();
  read_user_input();
}

fn say_hello(name: &str){
  println!("Hi there {}", name);
}

fn sample_declarations(){
  let sample_integer : u32 = 11;
  let sample_float : f32 = 11.5;
  let sample_string : &str = "The Rust programmng language";
  let fav_city : &str;
  const CAPITAL : &str = "New Delhi";
  println!("{} represents ints as {} and floats as {}", sample_string, sample_integer, sample_float);
  fav_city = "Mumbai";
  println!("My favourite city is {}", fav_city);
  println!("India's capital is {}", CAPITAL);
}

fn times_two(x : i32){
  println!("the output is: {}", x * 2)
}

// similar function but with a return call this time
fn times_three(x: i32) -> i32{
  return x * 3;
}

fn using_std_library(){
  let mut text = String::from("Hello");
  let mut message = text + " World";// using the std library allows us to combine strings (this is similar conceptually to the C++ std library)
  // you can also use functions like .push, etc. for strings using the String:: std lib component 
  println!("{}", message);
}

fn read_user_input(){
  let mut user_input = String::new();
  std::io::stdin().read_line(&mut user_input); //writes to the memory address of the variable
  println!("{}", user_input);

  // combine with error handling
  let mut user_age_input = String::new();
  println!("What's your age : ");
  std::io::stdin().read_line(&mut user_age_input);
  let mut user_age: u32 = user_age_input.trim().parse().unwrap();
  println!("you entered {} as your age.", user_age); 
}
