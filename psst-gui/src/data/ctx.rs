use std::fmt;


#[derive(Clone)]
pub struct Ctx<C, T> {
    pub ctx: C,
    pub data: T,
}

impl<C, T> Ctx<C, T> {
    pub fn new(c: C, t: T) -> Self {
        Self { ctx: c, data: t }
    }
}

impl<C, L> fmt::Debug for Ctx<C, L>
where
    L: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<C, L> PartialEq for Ctx<C, L>
where
    L: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}
