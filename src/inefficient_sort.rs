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

