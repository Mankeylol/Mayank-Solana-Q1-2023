pub fn run() {
  let person: (&str, &str, i8) = ("Brad", "England", 25);

  println!("{} is from {} and is {}", person.0, person.1, person.2);
}