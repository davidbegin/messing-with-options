#![allow(dead_code)]
#![allow(unused_variables)]

mod active_vector;

#[cfg(test)]
mod tests {
    use super::active_vector::shortest_toolbox::length;

    #[test]
    fn shortest_toolbox_length_returns_the_shortest_length() {
        let names = vec!["He", "Lee"];
        assert_eq!(length(names), 2);
    }

    #[test]
    fn shortest_toolbox_length_returns_the_shortest_length_2() {
        let names = vec!["Lee", "Smith"];
        assert_eq!(length(names), 3);
    }

    #[test]
    fn shortest_toolbox_length_returns_the_shortest_length_3() {
        let names = vec![];
        assert_eq!(length(names), 0);
    }
}


