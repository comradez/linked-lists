mod list;

#[cfg(test)]
mod tests {
    mod double_sided {
        use crate::list::double_sided::List;

        #[test]
        fn push_and_pop() {
            let mut list = List::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            println!("{}", list);
            println!("{}", list.pop_back().unwrap());
            println!("{}", list.pop_front().unwrap());
            println!("{}", list);
        }

        #[test]
        fn iter() {
            let mut list = List::new();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            let another_list: List<i32> = list.iter()
                .map(|val| 2 * val)
                .filter(|val| val < &5)
                .collect();
            println!("{}", another_list);
        }
    }

    mod single_sided {
        use crate::list::single_sided::List;
        
        #[test]
        fn push_and_pop() {
            let mut list = List::new();
            list.push_front(1);
            list.push_front(2);
            list.push_front(3);
            println!("{}", list.pop_front().unwrap());
            println!("{}", list.pop_front().unwrap());
        }

        #[test]
        fn iter() {
            let mut list = List::new();
            list.push_front(1);
            list.push_front(2);
            list.push_front(3);
            let another_list: Vec<i32> = list.iter()
                .map(|val| 2 * val)
                .filter(|val| val < &5)
                .collect();
            println!("{:?}", another_list);
        }
    }
}
