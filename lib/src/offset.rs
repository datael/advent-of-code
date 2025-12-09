use std::{
    mem::MaybeUninit,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Offset<T, const DIM: usize>(pub [T; DIM]);

impl<T> Offset<T, 2> {
    pub fn x(&self) -> &T {
        &self[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    pub fn y(&self) -> &T {
        &self[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }
}

impl<T> Offset<T, 3> {
    pub fn x(&self) -> &T {
        &self[0]
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    pub fn y(&self) -> &T {
        &self[1]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    pub fn z(&self) -> &T {
        &self[2]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }
}

impl<T, const DIM: usize> Offset<T, DIM> {
    pub fn from_comma_separated_str<'s>(value: &'s str) -> Result<Self, <T as FromStr>::Err>
    where
        T: FromStr,
    {
        let inputs = value.split(',').map(FromStr::from_str);

        let mut out: [MaybeUninit<T>; DIM] = [const { MaybeUninit::uninit() }; DIM];

        let mut n = 0;
        for input in inputs {
            match input {
                Ok(t) => out[n] = MaybeUninit::new(t),
                Err(err) => return Err(err),
            }
            n += 1;
        }

        assert_eq!(n, DIM);

        // SAFETY: If we got this far, we just initialized these above correctly.
        Ok(Offset::<T, DIM>(out.map(|d| unsafe { d.assume_init() })))
    }

    pub fn abs(&self) -> Self
    where
        T: Copy + Signed,
    {
        Self(self.map(|a| a.abs()))
    }
}

impl<T, const DIM: usize> Deref for Offset<T, DIM> {
    type Target = [T; DIM];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const DIM: usize> DerefMut for Offset<T, DIM> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_memberwise_binary_operator {
    ($trait:tt, $trait_fn:ident, $assigntrait:tt, $assign_trait_fn:ident, $op:tt) => {
        impl<T, const DIM: usize> $trait<Offset<T, DIM>> for Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Offset<T, DIM>;

            fn $trait_fn(mut self, rhs: Offset<T, DIM>) -> Self::Output {
                for i in 0..DIM {
                    self[i] = self[i] $op rhs[i]
                }

                self
            }
        }
        impl<T, const DIM: usize> $trait<&Offset<T, DIM>> for Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Offset<T, DIM>;

            fn $trait_fn(mut self, rhs: &Offset<T, DIM>) -> Self::Output {
                for i in 0..DIM {
                    self[i] = self[i] $op rhs[i]
                }

                self
            }
        }

        impl<T, const DIM: usize> $trait<Offset<T, DIM>> for &Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Offset<T, DIM>;

            fn $trait_fn(self, mut rhs: Offset<T, DIM>) -> Self::Output {
                for i in 0..DIM {
                    rhs[i] = self[i] $op rhs[i]
                }

                rhs
            }
        }

        impl<T, const DIM: usize> $trait<&Offset<T, DIM>> for &Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            type Output = Offset<T, DIM>;

            fn $trait_fn(self, rhs: &Offset<T, DIM>) -> Self::Output {
                let mut out: [MaybeUninit<T>; DIM] = [const { MaybeUninit::uninit() }; DIM];

                for i in 0..DIM {
                    out[i] = MaybeUninit::new(self[i] $op rhs[i]);
                }

                // SAFETY: We just initialized these above
                Offset::<T, DIM>(out.map(|d| unsafe { d.assume_init() }))
            }
        }

        impl<T, const DIM: usize> $assigntrait<Offset<T, DIM>> for Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            fn $assign_trait_fn(&mut self, rhs: Offset<T, DIM>) {
                *self = *self $op rhs;
            }
        }

        impl<T, const DIM: usize> $assigntrait<&Offset<T, DIM>> for Offset<T, DIM>
        where
            T: Copy + $trait<Output = T>,
        {
            fn $assign_trait_fn(&mut self, rhs: &Offset<T, DIM>) {
                *self = *self $op *rhs;
            }
        }
    };
}

impl_memberwise_binary_operator!(Add, add, AddAssign, add_assign, +);
impl_memberwise_binary_operator!(Sub, sub, SubAssign, sub_assign, -);
impl_memberwise_binary_operator!(Mul, mul, MulAssign, mul_assign, *);
impl_memberwise_binary_operator!(Div, div, DivAssign, div_assign, /);

impl<T, const DIM: usize> Offset<T, DIM> {
    #[must_use]
    pub fn manhattan_distance(&self, other: &Offset<T, DIM>) -> usize
    where
        T: Copy + CastToUsize + Sub<Output = T> + Signed,
    {
        let distances = self - other;

        let mut manhattan_distance = 0;
        for distance in distances.map(|d| d.abs()) {
            manhattan_distance += distance.as_usize();
        }

        manhattan_distance
    }

    /// ```
    /// # use advent_of_code_2025_lib::{Grid, Offset};
    ///
    /// let ed = Offset::<isize, 2>([0, 0]).euclidean_distance(&Offset::<isize, 2>([4, 3]));
    /// assert_eq!(ed, 5.);
    ///
    /// let ed = Offset::<isize, 3>([0, 0, 0]).euclidean_distance(&Offset::<isize, 3>([3, 4, 12]));
    /// assert_eq!(ed, 13.);
    /// ```
    #[must_use]
    pub fn euclidean_distance(&self, other: &Offset<T, DIM>) -> f64
    where
        T: Copy + CastToUsize + Sub<Output = T> + Signed,
    {
        let distances = self - other;

        let mut euclidean_distance = 0;
        for distance in distances.map(|d| d.abs()) {
            let as_usize = distance.as_usize();
            euclidean_distance += as_usize * as_usize;
        }

        (euclidean_distance as f64).sqrt()
    }

    /// ```
    /// # use advent_of_code_2025_lib::{Grid, Offset};
    ///
    /// let ed = Offset::<isize, 2>([0, 0]).euclidean_distance_sq(&Offset::<isize, 2>([4, 3]));
    /// assert_eq!(ed, 25);
    ///
    /// let ed = Offset::<isize, 3>([0, 0, 0]).euclidean_distance_sq(&Offset::<isize, 3>([3, 4, 12]));
    /// assert_eq!(ed, 169);
    /// ```
    #[must_use]
    pub fn euclidean_distance_sq(&self, other: &Offset<T, DIM>) -> usize
    where
        T: Copy + CastToUsize + Sub<Output = T> + Signed,
    {
        let distances = self - other;

        let mut euclidean_distance = 0;
        for distance in distances.map(|d| d.abs()) {
            let as_usize = distance.as_usize();
            euclidean_distance += as_usize * as_usize;
        }

        euclidean_distance
    }
}

pub trait Signed {
    fn abs(&self) -> Self;
}

macro_rules! impl_signed {
    ($signed:ty) => {
        impl Signed for $signed {
            fn abs(&self) -> Self {
                <$signed>::abs(*self)
            }
        }
    };
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);
impl_signed!(f32);
impl_signed!(f64);

pub trait CastToUsize {
    fn as_usize(&self) -> usize;
}

macro_rules! impl_cast_to_usize_i {
    ($type:ty) => {
        impl CastToUsize for $type {
            fn as_usize(&self) -> usize {
                self.cast_unsigned() as usize
            }
        }
    };
}

macro_rules! impl_cast_to_usize_f {
    ($type:ty) => {
        impl CastToUsize for $type {
            fn as_usize(&self) -> usize {
                self.abs() as usize
            }
        }
    };
}

impl_cast_to_usize_i!(i8);
impl_cast_to_usize_i!(i16);
impl_cast_to_usize_i!(i32);
impl_cast_to_usize_i!(i64);
impl_cast_to_usize_i!(i128);
impl_cast_to_usize_i!(isize);
impl_cast_to_usize_f!(f32);
impl_cast_to_usize_f!(f64);
