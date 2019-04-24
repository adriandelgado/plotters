use super::color::PlattleColor;

pub trait Plattle {
    const COLORS: &'static [(u8, u8, u8)];
    fn pick(idx: usize) -> PlattleColor<Self>
    where
        Self: Sized,
    {
        PlattleColor::<Self>::pick(idx)
    }
}

pub struct Plattle99;
pub struct Plattle9999;
pub struct Plattle100;

impl Plattle for Plattle99 {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (230, 25, 75),
        (60, 180, 75),
        (255, 225, 25),
        (0, 130, 200),
        (245, 130, 48),
        (145, 30, 180),
        (70, 240, 240),
        (240, 50, 230),
        (210, 245, 60),
        (250, 190, 190),
        (0, 128, 128),
        (230, 190, 255),
        (170, 110, 40),
        (255, 250, 200),
        (128, 0, 0),
        (170, 255, 195),
        (128, 128, 0),
        (255, 215, 180),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}

impl Plattle for Plattle9999 {
    const COLORS: &'static [(u8, u8, u8)] = &[
        (255, 225, 25),
        (0, 130, 200),
        (245, 130, 48),
        (250, 190, 190),
        (230, 190, 255),
        (128, 0, 0),
        (0, 0, 128),
        (128, 128, 128),
        (0, 0, 0),
    ];
}

impl Plattle for Plattle100 {
    const COLORS: &'static [(u8, u8, u8)] =
        &[(255, 225, 25), (0, 130, 200), (128, 128, 128), (0, 0, 0)];
}
