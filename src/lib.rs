fn josephus<T:Clone+Copy>(mut xs:Vec<T>,k:usize)-> Vec<T> {
    let mut re = Vec::with_capacity(xs.len());
    let k = k - 1;
    let mut index = 0;
    let mut get_indice = move |len| {index = (index+k) % len;index};
    while !xs.is_empty() {
        re.push(xs.remove(get_indice(xs.len())))
    }
    re
}

#[cfg(test)]
mod josephus {
    use super::*;

    #[test]
    fn test_works_with_integers() {
        assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(josephus(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2), vec![2, 4, 6, 8, 10, 3, 7, 1, 9, 5]);
    }
    #[test]
    fn test_works_with_strings() {
        assert_eq!(josephus("CodeWars".chars().collect::<Vec<char>>(), 4), "esWoCdra".chars().collect::<Vec<char>>());
    }
}
