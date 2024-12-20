

fn main(){
  print!("Create here either cli or server. These 2 tasks are optional\n");
  print!("You can create both or just one of them\n");
  println!("Or you can create something else, that you want to do\n");
  println!("Happy learning Rust!");
  
  // Task cli:
  // if you want, to create cli, that will read your name from arguments and print it
  // also, you can add flag to print your name in uppercase
  // also, you can add flag to print your name in lowercase
  // tip 0: use clap to parse arguments
  // tip 1: use this dialoguer to ask for your name if it is not provided
  // cargo add dialoguer
  // example: https://fadeevab.com/comparison-of-rust-cli-prompts/
  // tip 2: use pattern matching to check if flags are provided and in NONE case do Input::new() from dialoguer


  // Task server (harder)::
  // if you want, you can create a server, that will return users.json file content and write new user to it
  // tip 0: use axum to create server
  // tip 1: create users struct with nickname and role fields
  // tip 2:create create user endpoint that will accept json with nickname and role fields
  // tip 3: create get users endpoint that will return users.json file content
  // tip 4: you can use fs::read_to_string to read file content
  // tip 5: you can use serde_json::from_str to convert string to json
  // tip 6: you can use serde_json::to_string to write back to file
  // fs::write("users.json", serde_json::to_string(&users).unwrap()).unwrap();
}