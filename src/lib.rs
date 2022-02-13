//! My Linked List
//! 
//! 这个 crate 不会被发在 crates.io 上，所以我可以乱写，文档注释也没有。既然可以随意发挥，那我的小作文就写在这里了。
//! 如果不方便使用 `cargo doc` 构建文档，您也可以用任何的 Markdown 阅读器阅读这篇小作文。
//! 
//! 我一向觉得，情人节是和我无关的节日。恋爱中的男男女女相互交换情意，我只有在旁边艳羡的份。
//! 
//! 但今晚的感觉还是不太一样。和两位朋友各自聊了很久的天，然后看到了另一位朋友空间里发的非常甜的小作文，感觉突然就有了写点什么的冲动。
//! 最大的体会是，大家都有属于自己的幸福，也大多有独属于自己的痛苦——可能来自家庭，可能来自社会，也可能来自自己。也正应了那句话，幸福常常相似，而不幸各有不同。
//! 但爱的力量是强大的。这可能听起来相当*唯心*（或者*主观能动性*，如果你愿意这么解释XD），但爱中蕴藏的能量能够催生出强大的动力，指引我们在追寻幸福的道路上越走越远。
//! 
//! 今天是情人节，希望我在乎的人都能幸福。
//! 
//! 茨月
//! 
//! 2022.2.14 凌晨

pub use list::{List, Node};
pub use list::{IntoIter, Iter, IterMut};

mod list;

#[cfg(test)]
mod tests {
    use crate::list::List;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

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
