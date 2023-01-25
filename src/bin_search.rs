pub fn search<T: Eq + PartialOrd +  Copy> (arr: &[T], item: &T) -> Option<usize> {

    let mut min:usize = 0;
    let mut max:usize = arr.len();
    loop {
        
        if min==max {
            if arr[min] == *item {
                return Some(min);
            }
            else {
                return None;
            }
        }
        let mid:usize = (min+max)/2;
        if arr[mid] == *item {
            return Some(mid);
        } else {
            if arr[mid] > *item {
                max = mid;
            }
            else {
                min = mid+1;
                if min >= arr.len() {
                    return None;
                }
            }
        }

   }
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