#![no_std]

use core::mem::{self, MaybeUninit};
use core::slice;

#[derive(Debug)]
pub struct Oom;

pub struct Mem<'m> {
    raw: &'m mut [MaybeUninit<u8>],
}

impl<'m> Mem<'m> {
    /// # Errors
    pub fn with<T>(
        pool: &'m mut [MaybeUninit<u8>],
        f: impl FnOnce(Mem<'m>) -> Result<T, Oom>,
    ) -> Result<T, Oom> {
        f(Mem { raw: pool })
    }

    #[must_use]
    pub const fn free(&self) -> usize {
        self.raw.len()
    }
    
    /// # Errors
    #[cfg(feature = "unsound")]
    pub fn alloc_with_scratch<T>(
        &mut self,
        size: usize,
        f: impl FnOnce(&mut Mem<'m>, Mem<'_>) -> Result<&'m mut T, Oom>,
    ) -> Result<&'m mut T, Oom> {
        let free = self.free();
        if size > free {
            return Err(Oom);
        }

        let split_at = free - size;

        let raw = mem::take(&mut self.raw);

        let (main, scratch) = raw.split_at_mut(split_at);

        let mut main = Mem { raw: main };
        let result = f(&mut main, Mem { raw: scratch });

        let new_free = main.free();
        
        let raw = mem::take(&mut main.raw);    
        self.raw = unsafe { slice::from_raw_parts_mut(raw.as_mut_ptr(), new_free + size) };

        result
    }

    /// # Errors
    pub fn alloc<T>(&mut self, value: T) -> Result<&'m mut T, Oom> {
        self.align::<T>(1)?;

        let size = mem::size_of::<T>();

        let raw = mem::take(&mut self.raw);
        let (allocated, raw) = raw.split_at_mut(size);

        let allocated = unsafe { &mut *allocated.as_mut_ptr().cast::<MaybeUninit<T>>() };
        let result = allocated.write(value);

        self.raw = raw;

        Ok(result)
    }

    /// # Errors
    pub fn alloc_default<T: Default>(&mut self) -> Result<&'m mut T, Oom> {
        self.alloc(T::default())
    }

    /// # Errors
    pub fn array_alloc<T>(
        &mut self,
        len: usize,
        mut init: impl FnMut(usize) -> T,
    ) -> Result<&'m mut [T], Oom> {
        self.align::<T>(len)?;

        let size = mem::size_of::<T>();

        let raw = mem::take(&mut self.raw);
        let (allocated, raw) = raw.split_at_mut(size * len);
        {
            let mut ptr = allocated.as_mut_ptr();
            for i in 0..len {
                unsafe { &mut *ptr.cast::<MaybeUninit<T>>() }.write(init(i));
                ptr = unsafe { ptr.add(size) };
            }
        }

        self.raw = raw;

        Ok(unsafe { slice::from_raw_parts_mut(allocated.as_mut_ptr().cast::<T>(), len) })
    }

    /// # Errors
    pub fn array_alloc_default<T: Default>(&mut self, len: usize) -> Result<&'m mut [T], Oom> {
        self.array_alloc(len, |_| T::default())
    }

    /// # Errors
    pub fn array_collect_alloc<U, T, I, F>(
        &mut self,
        len: usize,
        iter: I,
        f: F,
    ) -> Result<&'m mut [T], Oom>
    where
        I: Iterator<Item = U>,
        F: Fn(&mut Mem<'m>, U) -> Result<T, Oom>,
    {
        self.align::<T>(len)?;

        let size = mem::size_of::<T>();

        let raw = mem::take(&mut self.raw);
        let (allocated, raw) = raw.split_at_mut(size * len);

        let mut main = Mem { raw };

        let mut oom = false;
        
        let mut ptr = allocated.as_mut_ptr();

        let mut count = 0;
        for (i, value) in iter.enumerate() {
            count += 1;
            if i >= len {
                oom = true;
                break;
            }

            if let Ok(value) = f(&mut main, value) {
                unsafe { &mut *ptr.cast::<MaybeUninit<T>>()}.write(value);
                ptr = unsafe { ptr.add(size) };
            } else {
                oom = true;
                break;
            }
        }

        let raw = mem::take(&mut main.raw);
        
        self.raw = raw;

        if oom {
            Err(Oom)
        } else {
            Ok(unsafe { slice::from_raw_parts_mut(allocated.as_mut_ptr().cast::<T>(), count) })
        }
    }

    fn align<T>(&mut self, len: usize) -> Result<(), Oom> {
        let size = mem::size_of::<T>();
        if size * len > self.free() {
            return Err(Oom);
        }

        let align = mem::align_of::<T>();

        let ptr = self.raw.as_mut_ptr();

        let offset = ptr.align_offset(align);
        if offset >= self.free() || self.free() - offset < size * len {
            return Err(Oom);
        }

        let raw = mem::take(&mut self.raw);
        let (_, raw) = raw.split_at_mut(offset);

        self.raw = raw;

        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::cast_possible_truncation)]
mod test {
    use super::*;

    #[test]
    fn test_u32() {
        let mut pool = [MaybeUninit::uninit(); mem::size_of::<u32>() * 3];

        let result = Mem::with(&mut pool, |mut mem| {
            let x = mem.alloc(1u32)?;
            let y = mem.alloc(2u32)?;
            Ok(*x + *y)
        })
        .unwrap();

        assert_eq!(result, 3u32);
    }

    #[test]
    fn test_u8() {
        let mut pool = [MaybeUninit::uninit(); 2];

        let result = Mem::with(&mut pool, |mut mem| {
            let x = mem.alloc(1u8)?;
            let y = mem.alloc(2u8)?;
            Ok(*x + *y)
        })
        .unwrap();

        assert_eq!(result, 3u8);

        assert_eq!(unsafe { pool[0].assume_init_read() }, 1);
        assert_eq!(unsafe { pool[1].assume_init_read() }, 2);
    }

    #[test]
    fn test_u8_mut() {
        let mut pool = [MaybeUninit::uninit(); 2];

        let result = Mem::with(&mut pool, |mut mem| {
            let x = mem.alloc_default()?;
            let y = mem.alloc_default()?;

            *x = 1u8;
            *y = 2u8;

            Ok(*x + *y)
        })
        .unwrap();

        assert_eq!(result, 3u8);

        assert_eq!(unsafe { pool[0].assume_init_read() }, 1);
        assert_eq!(unsafe { pool[1].assume_init_read() }, 2);
    }

    #[test]
    fn test_u8_2_3() {
        let mut pool = [MaybeUninit::uninit(); 2 + 3];

        let result: u8 = Mem::with(&mut pool, |mut mem| {
            let arr_2 = mem.array_alloc(2, |i| (i + 1) as u8)?;
            let arr_3 = mem.array_alloc(3, |i| (i + 10) as u8)?;
            Ok(arr_2.iter().sum::<u8>() + arr_3.iter().sum::<u8>())
        })
        .unwrap();

        assert_eq!(result, 36);

        assert_eq!(unsafe { pool[0].assume_init_read() }, 1);
        assert_eq!(unsafe { pool[1].assume_init_read() }, 2);

        assert_eq!(unsafe { pool[2].assume_init_read() }, 10);
        assert_eq!(unsafe { pool[3].assume_init_read() }, 11);
        assert_eq!(unsafe { pool[4].assume_init_read() }, 12);
    }

    #[test]
    fn test_u8_mut_2_3() {
        let mut pool = [MaybeUninit::uninit(); 2 + 3];

        let result: u8 = Mem::with(&mut pool, |mut mem| {
            let arr_2 = mem.array_alloc_default(2)?;
            let arr_3 = mem.array_alloc_default(3)?;

            for (i, v) in arr_2.iter_mut().enumerate() {
                *v = (i + 1) as u8;
            }

            for (i, v) in arr_3.iter_mut().enumerate() {
                *v = (i + 10) as u8;
            }

            Ok(arr_2.iter().sum::<u8>() + arr_3.iter().sum::<u8>())
        })
        .unwrap();

        assert_eq!(result, 36);

        assert_eq!(unsafe { pool[0].assume_init_read() }, 1);
        assert_eq!(unsafe { pool[1].assume_init_read() }, 2);

        assert_eq!(unsafe { pool[2].assume_init_read() }, 10);
        assert_eq!(unsafe { pool[3].assume_init_read() }, 11);
        assert_eq!(unsafe { pool[4].assume_init_read() }, 12);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "unsound")]
    fn test_alloc_with_scratch() {
        let mut pool = [MaybeUninit::uninit(); 4];

        let result = Mem::with(&mut pool, |mut mem| {
            let result = mem
                .alloc_with_scratch(mem.free() - 1, |mem, mut scratch| {
                    assert_eq!(mem.free(), 1);
                    assert_eq!(scratch.free(), 3);

                    let arr = scratch.array_alloc_default(scratch.free()).unwrap();
                    for (i, v) in arr.iter_mut().enumerate() {
                        *v = (i + 1) as u8;
                    }

                    let r = mem.alloc(42u8 + arr.iter().sum::<u8>());

                    r
                })
                .unwrap();
            assert_eq!(mem.free(), 3);
            Ok(*result)
        })
        .unwrap();

        assert_eq!(result, 48);
    }

    #[test]
    #[cfg_attr(miri, ignore)]
    #[cfg(feature = "unsound")]
    fn test_alloc_with_scratch_simple() {
        let mut pool = [MaybeUninit::uninit(); 4];

        let result = Mem::with(&mut pool, |mut mem| {
            let result = mem
                .alloc_with_scratch(mem.free() - 2, |mem, scratch| {
                    assert_eq!(mem.free(), 2);
                    assert_eq!(scratch.free(), 2);

                    let r = mem.alloc(42u8);

                    let _ = mem.alloc(42u8);

                    r
                })
                .unwrap();
            assert_eq!(mem.free(), 2);
            Ok(*result)
        })
        .unwrap();

        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_struct() {
        #[derive(PartialEq, Debug)]
        struct Tree<'a> {
            value: u8,
            left: Option<&'a Tree<'a>>,
            right: Option<&'a Tree<'a>>,
        }

        let mut pool = [MaybeUninit::uninit(); mem::size_of::<Tree>() * 5];
        let result = Mem::with(&mut pool, |mut mem| {
            let l_left = mem.alloc(Tree {
                value: 1,
                left: None,
                right: None,
            })?;
            let l_right = mem.alloc(Tree {
                value: 2,
                left: None,
                right: None,
            })?;
            let left = mem.alloc(Tree {
                value: 3,
                left: Some(l_left),
                right: Some(l_right),
            })?;
            let right = mem.alloc(Tree {
                value: 4,
                left: None,
                right: None,
            })?;

            Ok(Tree {
                value: 0,
                left: Some(left),
                right: Some(right),
            })
        })
        .unwrap();

        assert_eq!(result.value, 0);
        assert_eq!(result.left.unwrap().value, 3);
        assert_eq!(result.left.unwrap().left.unwrap().value, 1);
        assert_eq!(result.left.unwrap().right.unwrap().value, 2);
        assert_eq!(result.right.unwrap().value, 4);
    }

    #[test]
    fn test_array_collect_alloc_simple() {
        let mut pool = [MaybeUninit::uninit(); const { mem::size_of::<usize>() * 10 + mem::size_of::<u8>() * 10 }];

        let _ = Mem::with(
            &mut pool,
            |mut mem| {
                let dummy = mem.alloc(());
                
                let result = mem.array_collect_alloc(
                    10,
                    1..=5,
                    |_, value| {
                        Ok(value)
                    },
                ).unwrap();
                
                assert_eq!(result, [1, 2, 3, 4, 5].as_slice());

                Ok(dummy)
            },
        ).unwrap();
    }

    #[test]
    fn test_array_collect_alloc() {
        let mut pool = [MaybeUninit::uninit(); const { mem::size_of::<usize>() * 10 + mem::size_of::<u64>() * 10 }];

        let _ = Mem::with(
            &mut pool,
            |mut mem| {
                let dummy = mem.alloc(());
                
                let result = mem.array_collect_alloc(
                    10,
                    r"1
2
3
4
5".lines().map(|line| line.parse::<u8>().unwrap()),

                    |mem, value| {
                        mem.alloc(u64::from(value) * 2)
                    },
                ).unwrap();

                assert_eq!(result, [&2, &4, &6, &8, &10].as_slice());

                Ok(dummy)
            },
        ).unwrap();
    }
}
