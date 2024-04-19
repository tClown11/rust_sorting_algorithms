use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
     where
        T: Ord,
    {
        // { sorted | not sorted }
        for unsorted in 0..slice.len() {
            // 函数调用的方式
           let smallest_in_rest= slice[unsorted..]
               .iter()
               .enumerate()
               .min_by_key(|&(_, v)| v)
               .map(|(i, _)| unsorted + i)
               .expect("slice is no-empty");

            // // or
            // 使用 for 循环的方式
            //
            // let mut smallest_in_rest = unsorted;
            // for i in (unsorted+1)..slice.len() {
            //     if slice[i] < slice[smallest_in_rest] {
            //         smallest_in_rest = i;
            //     }
            // }

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}