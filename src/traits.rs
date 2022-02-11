use glam::{Quat, Vec2, Vec3, Vec4};

pub trait Lerp<'a> {
    type Write;
    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32);
}

impl<'a> Lerp<'a> for f32 {
    type Write = &'a mut Self;

    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
        *write = (1.0 - t) * start + t * end
    }
}

macro_rules! mul_impl {
    ($ty: ident) => {
        impl<'a> Lerp<'a> for $ty {
            type Write = &'a mut Self;

            fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
                *write = ((1.0 - t) * *start + t * *end ) as Self
            }
        }
    };
    ([$($ty: ident),*]) => {
        $(
            mul_impl!($ty);
        )*
    };
}

mul_impl!([Vec2, Vec3, Vec4]);

impl<'a> Lerp<'a> for Quat {
    type Write = &'a mut Self;

    fn lerp(write: Self::Write, start: &Self, end: &Self, t: f32) {
        *write = start.slerp(*end, t)
    }
}

macro_rules! impl_tuples {
    () => {};
    ($([$idx: tt => $name: ident]), *) => {
        impl<'a, $($name: Lerp<'a> + 'a),*> Lerp<'a> for ($($name,)*) {
        type Write = ($(<$name as Lerp<'a>>::Write,)*);
            fn lerp(write: Self::Write, start: &Self,end: &Self, t: f32) {
                $(
                 $name::lerp(write.$idx, &start.$idx, &end.$idx, t);
                ) *
            }
        }
    };
}

impl_tuples!([0 => A]);
impl_tuples!([0 => A], [1 => B]);
impl_tuples!([0 => A], [1 => B], [2 => C]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F], [6 => G]);
impl_tuples!([0 => A], [1 => B], [2 => C], [3 => D], [4 => E], [5 => F], [6 => G], [7 => H]);
