//! # simple shell
//!
//! > author: Wyatt Belcher
//!

use std::io::{
    self,
    Write,
};
use std::process::Command;

fn main(){
    println!(r#"
                                                   
      ▒▒▒▒                                         
    ▒▒                                             
    ▒▒            ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒     
      ▒▒▒▒▒▒▒▒▒▒▒▒                            ▓▓   
      ░░░░░░                                  ▒▒    
                                              ▒▒    
              ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▒▒  ▒▒      
          ░░▒▒░░░░░░░░░░░░░░▓▓▓▓▒▒░░░░▒▒▒▒▒▒▒▒      
        ▒▒▒▒▒▒▒▒░░▒▒░░░░▒▒▒▒▒▒▓▓▒▒▒▒▒▒░░▒▒▒▒        
      ▒▒▒▒░░▒▒▒▒░░░░▒▒░░░░▒▒░░▒▒▒▒░░▒▒░░             
    ▒▒▒▒░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒░░▒▒▒▒▒▒                
    ▒▒▒▒▒▒▒▒▒▒░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒                       
  ▒▒▒▒  ▒▒▒▒░░▒▒▒▒▒▒░░    ▒▒    ▒▒                     
  ▒▒▒▒░░▒▒▒▒▒▒▒▒░░▒▒▒▒░░        ░░▒▒▒▒▒▒▒▒             
  ▒▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░  ▒▒      ▒▒▒▒▒▒▒▒▒▒             
  ▒▒▒▒▒▒▒▒░░▒▒▒▒▒▒  ░░▒▒      ▒▒▒▒░░▒▒▒▒               
  ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒░░▒▒  ▒▒▒▒░░▒▒▒▒                 
  ▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒                   
  ▒▒░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░▒▒                  
    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒▒░░▒▒▒▒▒▒▒▒                       
      ▒▒▒▒▒▒░░░░░░░░▒▒░░▒▒▒▒▒▒                         
          ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒                              
                                                        
                                                      "#);
    
    loop {
        print!("> ");
        // this just makes sure the prompt gets displayed
        io::stdout().flush().unwrap();

        let mut command : String = String::new();
        // get the user's imput
        io::stdin().read_line(&mut command).unwrap();

        let mut parts = command.split_whitespace();

        let Some(program) = parts.next() else {
            continue;
        };

        if program == "exit" {
            break;
        }

        let output = match Command::new(program).args(parts).output() {
            Ok(output) => output,
            Err(error) => {
                eprintln!("{program}: {error}");
                continue;
            }
        };

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
        
}
