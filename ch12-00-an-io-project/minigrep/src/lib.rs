use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

// タプルでも返せたが、意味を持つために構造体を作成
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // argsの最初の引数は、 `target/debug/minigrep` のため、3つ以上が必要
            // panic!("not enough arguments");
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
      println!("{}", line)
    }

    // Ok(()) という記法は副作用のためだけに呼び出している、と示唆する慣習的な方法
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

// #[cfg(test)] という注釈は、コンパイラに cargo build を走らせた時ではなく、 cargo test を走らせた時にだけ
// テストコードをコンパイルし走らせるよう指示する(ライブラリをビルドしたいだけの時のコンパイルタイムと成果物のサイズを節約)
#[cfg(test)]
mod tests {
    use super::*;

  #[test]
  fn args_is_shorter_than_3 () {
    let args = vec![String::from("arg1"), String::from("arg2")];
    let result = Config::new(&args);
    assert_eq!(result.err().unwrap(), "not enough arguments");
  }

  #[test]
  fn args_is_3 () {
    let args = vec![String::from("arg1"), String::from("arg2"), String::from("arg3")];
    let result = Config::new(&args);
    assert_eq!(result.is_ok(), true);
  }  

  #[test]
  fn args_is_longer_than_3 () {
    let args = vec![String::from("arg1"), String::from("arg2"), String::from("arg3"), String::from("arg4")];
    let result = Config::new(&args);
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );

  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    // 私を信じて
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)

    )
  }
}