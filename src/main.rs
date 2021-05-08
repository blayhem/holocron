use same_file::Handle;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, stdin, stdout};
use std::path::Path;

fn help() {
  println!("WIP atm please introduce some args");
}

fn read_and_save(path: &Path) {
    let mut s=String::new();
    print!("Please enter some text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    save(path, s);
}

fn save(path: &Path, content: String) {
    let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(path)
      .unwrap();

    if let Err(e) = writeln!(file, "{:?}", content) {
      eprintln!("Couldn't write to file: {}", e);
    }

    println!("\nCool! Saved {} in {}", content, path.display());
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
        "save" => {
            read_and_save(path_to_read);
        }
        _ => println!("Wrong args"),
      },
      3 => {
        match args[1].as_str() {
          "save" => {
              save(path_to_read, args[2].clone())
          }
          _ => println!("Wrong args"),
        }
      }
      _ => help(),
    }
  }

  Ok(())
}
