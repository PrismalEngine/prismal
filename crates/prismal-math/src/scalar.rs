use num_traits::{AsPrimitive, NumOps, One, Zero};

pub trait Scalar: Zero + One + NumOps + PartialEq + Copy {
    #[inline]
    fn lerp(self, other: Self, factor: f32) -> Self
    where
        Self: 'static,
        Self: AsPrimitive<f32>,
        f32: AsPrimitive<Self>,
    {
        (self.as_() + factor * (other - self).as_()).as_()
    }
    #[inline]
    fn lerp_factor(self, other: Self, output: Self) -> f32
    where
        Self: 'static,
        Self: AsPrimitive<f32>,
        f32: AsPrimitive<Self>,
    {
        let num: f32 = (output - self).as_();
        let den: f32 = (other - self).as_();
        num / den
    }

    #[inline]
    fn remap(self, lo_in: Self, hi_in: Self, lo_out: Self, hi_out: Self) -> Self
    where
        Self: 'static,
        Self: AsPrimitive<f32>,
        f32: AsPrimitive<Self>,
    {
        let factor = lo_in.lerp_factor(hi_in, self);
        lo_out.lerp(hi_out, factor)
    }
}

impl Scalar for f32 {}
impl Scalar for f64 {}

impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
