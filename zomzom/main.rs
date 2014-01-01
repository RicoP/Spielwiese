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
  let game : Option<@GameContext> = None; 

  loop {
    match state {
      Menu => { 
        (state, game) = menuLoop(); 
      },
      Game => { 
         match game {
          Some(g) => {            
            state = gameLoop(g);
          },
          None => fail!("")
        }
      },
      Quit => { return; }
    }
  }
}

fn menuLoop() -> (GlobalState, Option<@GameContext>) {
  let reader = std::io::stdin(); 

  println("Enter start, load or quit."); 

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

fn gameLoop(game : @GameContext) -> GlobalState{
  let reader = std::io::stdin(); 
  print(fmt!("Starting game %s\n", game.name));

  loop { 
    let txt = reader.read_line(); 

    match txt {
      ~"quit" => return Quit,  
      _ => print(fmt!("%s %s\n", "hello", txt))
    }
  }
}
