enum GlobalState {
  Menu, 
  Game,
  Quit
}

struct GameContext {
  name: ~str 
}

fn main() {
  let mut state : GlobalState = Menu; 
  let mut game : @GameContext = @GameContext { name : ~""}; 

  loop {
    let opt : Option<@GameContext> = None; 
    match state {
      Menu => { 
        (state, opt) = menuLoop(); 

        match opt {
          Some(ctx) => game = ctx,
          _ => fail!("")
        }
      },
      Game => { /*gameLoop(game)*/ },
      Quit => { return; }
    }

    match opt {
      Some(ctx) => game = ctx,
      _ => fail!("")
    }
  }
}

fn menuLoop() -> (GlobalState, Option<@GameContext>) {
  let reader = std::io::stdin(); 

  println("Enter quit, load or quit."); 

  loop { 
    let txt = reader.read_line(); 

    match txt {
      ~"start" => {
        let ctx = @GameContext { name : ~"new" }; 

        return (Game, Some(ctx));
      }
      ~"load" => {
        let ctx = @GameContext { name : ~"load" }; 

        return (Game, Some(ctx));
      }
      ~"quit" => {
        return (Quit, None);     
      }
      _ => print(fmt!("Unknown command '%s'.\n", txt))
    }
  }
}

fn gameLoop(game : @GameContext) {
  let reader = std::io::stdin(); 
  loop { 
    let txt = reader.read_line(); 

    match txt {
      ~"quit" => return,  
      _ => print(fmt!("%s %s\n", "hello", txt))
    }
  }
}
