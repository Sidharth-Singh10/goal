use std::env;
use std::process;

use goal::print_breaking;
// use goal::print_transfers;
use tables::choose_table;
// use tries::print_table;
mod tables;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Arre batao tho, chahiye kya");
        process::exit(1);
    }

    if args.len() == 2 {
        let command = &args[1];

        match &command[..] {
          "breaking" => {
              if let Err(err) = print_breaking() {
                  eprintln!("Error printing breaking news: {:?}", err);
              }
          },
          // "transfers" => print_transfers(),
          _ => println!("itna tho hme bhi nahi pta"),
      }
      
    }

    if args.len() == 3
    {
      let command1 = &args[1];

      match &command1[..]
      {
        "table" =>
        {
          let command2 = &args[2];
          match &command2[..]
          {
            "laliga" => choose_table("laliga"),
            "premier" => choose_table("premier"),
            "serieA" => choose_table("serieA"),
            "ligue1" => choose_table("ligue1"),
            "bundesliga" => choose_table("bundesliga"),
            "dutch" => choose_table("dutch"),
            _  => println!("Google karle"),

          }
        },

        _ => println!("Manne na pata"),
      }

      
    }


}
