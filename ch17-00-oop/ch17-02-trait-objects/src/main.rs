extern crate ch17_02_trait_objects;
use ch17_02_trait_objects::{Draw, Button, Screen};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
      println!("draw with SelectBox");
  }
}

fn main() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("maybe"),
          String::from("no"),
        ]
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("Button OK")
      })
    ]
  };

  screen.run();
}