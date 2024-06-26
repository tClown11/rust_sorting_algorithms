use super::Sorter;

pub struct QuickSort;


fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len()-1;
    while left <= right {
        if &rest[left] <= pivot {
            // already on the correct side
            left += 1;
        }else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                break;
            }
            right -= 1
        } else {
            // move element to the right side
            rest.swap(left, right);
            left += 1;
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // place the pivot at its final location
    slice.swap(0, left);

    let (left, right) = slice.split_at_mut(left);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
}
impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
        where
            T: Ord,
    {
       // [unsorted | pivot | unsorted]

        quicksort(slice)
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}