fn main() {
  let reader = std::io::stdin(); 
  loop { 
    let txt = reader.read_line(); 

    match txt {
      ~"quit" => return,  
      _ => print(fmt!("%s %s\n", "hello", txt))
    }
  }
}
