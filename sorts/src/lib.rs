fn sort_insertion<T:Ord>(arr : &mut [T]) {
    if arr.len() < 2 { return; }
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && (arr[j-1] > arr[j]) {
            arr.swap(j-1, j);
            j -= 1;
        }
    }
}

fn sort_selection<T:Ord>(arr : &mut [T]) {
    if arr.len() < 2 { return; }
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i+1..arr.len() {
            if arr[min_index] > arr[j] { min_index = j; }
        }
        arr.swap(i, min_index);
    }
}

fn sort_bubble<T:Ord>(arr : &mut [T]) {
    if arr.len() < 2 { return; }
    let mut swapped = false;
    for i in 0..arr.len() {
        for j in 1..arr.len()-i {
            if arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                swapped = true;
            }
        }
        if swapped == false  { break; }
    }
}

fn sort_merge<T:Ord>(arr : &mut [T]) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_unsorted() -> Vec<u32> {
        let vec = vec![7,4,3,5,8,2,4,8,4,1,9];
        let mut vector = Vec::new();
            for i in vec.iter() {
            vector.push(*i);
        }
        vector
    }

    #[test]
    fn test_sort_insertion() {
        let mut sort_me = get_unsorted();
        println!("unsorted:\n{:?}", sort_me);
        sort_insertion(&mut sort_me);
        println!("sorted:\n{:?}", sort_me);
        assert_eq!(sort_me, vec![1,2,3,4,4,4,5,7,8,8,9]);
    }

    #[test]
    fn test_sort_selection() {
        let mut sort_me = get_unsorted();
        println!("unsorted:\n{:?}", sort_me);
        sort_selection(&mut sort_me);
        println!("sorted:\n{:?}", sort_me);
        assert_eq!(sort_me, vec![1,2,3,4,4,4,5,7,8,8,9]);
    }

    #[test]
    fn test_sort_bubble() {
        let mut sort_me = get_unsorted();
        println!("unsorted:\n{:?}", sort_me);
        sort_bubble(&mut sort_me);
        println!("sorted:\n{:?}", sort_me);
        assert_eq!(sort_me, vec![1,2,3,4,4,4,5,7,8,8,9]);
    }
}
