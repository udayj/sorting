pub fn search<T: Eq> (arr: &[T], item: &T) -> Option<usize> {

    for (i, x) in arr.iter().enumerate() {

        if  x == item {
            return Some(i)
        }
    }
    None
}

#[cfg(test)]
mod test
{
    use super::search;

    #[test]
    fn basic_search() {

        let arr = vec!(1,2,3,4,5);
        assert_eq!(search(&arr,&3), Some(2));
        assert_eq!(search(&arr,&6), None);
    }
}