pub mod longest_toolbox {
    pub fn length(vec: Vec<&str>) -> usize {
        longest_option(vec).unwrap_or("").len()
    }

    fn longest_option(vec: Vec<&str>) -> Option<&str> {
        if vec.len() > 0 {
            let mut longest = vec[0];
            for element in vec.iter() {
                if element.len() > longest.len() {
                    longest = *element;
                }
            }
            Some(longest)
        } else {
            None
        }
    }
}

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
