pub fn run() {
    let name = "Mayank";
    let mut age = 18;
    println!("My name is {} and I am {}", name, age);
    age = 19;
    println!("My name is {} and I am {}", name, age);
  
    // Define constant
    const ID: i32 = 007;
    println!("ID: {}", ID);
  
    // Assign multiple vars
    let ( my_name, my_age ) = ("Mayank", 18);
    println!("{} is {}", my_name, my_age );
  }