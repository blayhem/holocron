use same_file::Handle;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::Path;

fn help() {
  println!("WIP atm please introduce some args");
}

fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let path_to_read = Path::new("lines.txt");
  let stdout_handle = Handle::stdout()?;
  let handle = Handle::from_path(path_to_read)?;

  if stdout_handle == handle {
    return Err(Error::new(
      ErrorKind::Other,
      "You are reading and writing to the same file",
    ));
  } else {
    match args.len() {
      1 => help(),
      2 => match args[1].as_str() {
        "ls" => {
          let file = File::open(&path_to_read)?;
          let file = BufReader::new(file);
          for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?);
          }
        }
        _ => println!("Wrong args"),
      },
      3 => {
        match args[1].as_str() {
          "save" => {
            let mut file = OpenOptions::new()
              .write(true)
              .append(true)
              .open(path_to_read)
              .unwrap();

            if let Err(e) = writeln!(file, "{:?}", args[2]) {
              eprintln!("Couldn't write to file: {}", e);
            }
          }
          _ => println!("Wrong args"),
        }
      }
      _ => help(),
    }
  }

  Ok(())
}
