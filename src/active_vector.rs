pub mod shortest_toolbox {
    pub fn print(names: Vec<&str>) {
        println!("Shortest Name: {:?}", name(names));
    }

    fn name(names: Vec<&str>) -> String {
        match option(names) {
            Some(shortest) => shortest.to_string(),
            _              => "Not Found".to_string(),
        }
    }

    fn option(names: Vec<&str>) -> Option<&str> {
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
