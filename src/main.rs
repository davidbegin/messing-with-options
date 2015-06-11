#![allow(dead_code)]
#![allow(unused_variables)]

mod active_title;
mod active_vector;

fn main() {
    active_title::print();

    // let names = vec!["Smith", "Lee", "He"];
    // active_vector::shortest_toolbox::print(names);
    // let other_names = vec![];
    // active_vector::shortest_toolbox::print(other_names);

    if_let_madness();
}

fn if_let_madness() {
  // match option {
  //   Some(x) => { foo(x) },
  //   None => {},
  // }

  // we have an option
  // we want to call a method on that option,
  // ONLY if it is some

  let option = Some(1);

  match option {
    Some(x) => { print_thang(x) },
    None => {},
  }

  // so this blank None branch seems like a waste
  // how else can we write this?

  let option = Some(2);

  if option.is_some() {
    let x = option.unwrap();
    print_thang(x);
  }

  // even shorter?
  let option = Some(3);

  if option.is_some() {
    print_thang(option.unwrap());
  }

  // now with if let!

  let option = Some(4);

  if let Some(x) = option {
    print_thang(x);
  }
}

fn print_thang(thang: i32) {
    println!("Heres that thang: {}", thang);
}
