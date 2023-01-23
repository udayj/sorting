pub fn sort<T: Ord + Copy> (arr: &mut [T]) {

    if arr.len() <=1 {
        return;
    }
    let mid:usize = arr.len()/2;
    sort(&mut arr[..mid]);
    sort(&mut arr[mid..]);
    merge(arr,mid);
    return;
}

//
fn merge<T: Ord + Copy> (arr: &mut [T], mid: usize) {

    let length = arr.len();
    let left_arr = arr[..mid].to_vec();
    let right_arr = arr[mid..].to_vec();
    let mut j:usize=0;
    let mut k:usize=0;
    for i in 0..length {

        if k==right_arr.len() || (left_arr[j] < right_arr [k] && j<mid) {
            arr[i] = left_arr[j];
            j+=1;
        } else {
            arr[i] = right_arr[k];
            k+=1;
        }
    }
    return;
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