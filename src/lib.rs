pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where 
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod heapsort;
mod radixsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;
pub use radixsort::RadixSort;
pub use radixsort::Bytify;
pub use heapsort::HeapSort;


pub struct StdSorter;
impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;
impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
        where
            T: Ord,
    {
        slice.sort_unstable();
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1,2,3,4]);
    }
}