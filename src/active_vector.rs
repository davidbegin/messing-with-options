pub mod shortest_toolbox {
    pub fn length(names: Vec<&str>) -> usize {
        shortest_option(names).unwrap_or("").len()
    }

    pub fn print(names: Vec<&str>) {
        println!("{:?}", shortest_option(names).unwrap_or("Not Found"));
    }

    fn shortest_option(names: Vec<&str>) -> Option<&str> {
        if names.len() > 0 {
            let mut shortest = names[0];
            for name in names.iter() {
                if name.len() < shortest.len() {
                    shortest = *name;
                }
            }
            Some(shortest)
        } else {
            None
        }
    }
}
