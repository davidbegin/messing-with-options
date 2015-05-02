mod active_title;
mod active_vector;

fn main() {
    active_title::print();

    let names = vec!["Smith", "Lee", "He"];
    println!("Shortest Name: {:?}", active_vector::get_shortest(names).unwrap());

}
