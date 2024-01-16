#![no_std]

use core::mem::MaybeUninit;

pub trait CollectArray: Iterator {
    fn collect_array<const N: usize>(mut self) -> Option<[Self::Item; N]>
    where
        Self: Sized,
    {
        let mut arr: [MaybeUninit<Self::Item>; N] = core::array::from_fn(|_| MaybeUninit::uninit());
        for i in 0..N {
            arr[i] = match self.next().map(MaybeUninit::new) {
                Some(value) => value,
                None => {
                    for j in 0..i {
                        unsafe {
                            arr[j].as_mut_ptr().drop_in_place();
                        }
                    }
                    return None;
                }
            };
        }
        let arr = arr.map(|x| unsafe { x.assume_init() });
        if self.next().is_some() {
            return None;
        }
        Some(arr)
    }
}

impl<It> CollectArray for It where It: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_array() {
        let arr = (0..5).collect_array::<5>().unwrap();
        let mut iter = arr.iter().copied();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_collect_array_fail_if_bigger() {
        let iter = (0..10).collect_array::<11>();
        assert!(iter.is_none());
    }

    #[test]
    fn test_collect_array_fail_if_smaller() {
        let iter = (0..10).collect_array::<9>();
        assert!(iter.is_none());
    }
}
