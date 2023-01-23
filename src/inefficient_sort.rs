pub fn sort<T:Ord> (arr: &mut [T]) {

    let length=arr.len();
    loop {
        let mut sorted=true;
        for i in 0..(length-1) {
            if arr[i]>arr[i+1] {
                arr.swap(i,i+1);
                sorted=false;
            }
        }
        if sorted == true {
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::sort;
    #[test]
    fn basic_test() {
        let mut x = [5,4,3,2,1];
        sort(&mut x);
        assert_eq!(x,[1,2,3,4,5]);
    }
}