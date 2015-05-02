#![allow(dead_code)]
#![allow(unused_variables)]

mod active_vector;

#[cfg(test)]
mod tests {

    mod longest_toolbox_tests {
        use active_vector::longest_toolbox::length;

        #[test]
        fn length_returns_the_longest_length() {
            let vec = vec!["He", "Lee"];
            assert_eq!(length(vec), 3);
        }

        #[test]
        fn length_returns_0_if_the_vector_is_empty() {
            let vec = vec![];
            assert_eq!(length(vec), 0);
        }
    }

    mod shortest_toolbox_tests {
        use active_vector::shortest_toolbox::length;

        #[test]
        fn length_returns_the_shortest_length() {
            let names = vec!["He", "Lee"];
            assert_eq!(length(names), 2);
        }

        #[test]
        fn length_returns_0_if_the_vector_is_empty() {
            let names = vec![];
            assert_eq!(length(names), 0);
        }
    }
}


