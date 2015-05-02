mod active_title;
mod active_vector;

fn main() {
    active_title::print();

    let names = vec!["Smith", "Lee", "He"];
    active_vector::shortest_toolbox::print(names);
    let other_names = vec![];
    active_vector::shortest_toolbox::print(other_names);
}
