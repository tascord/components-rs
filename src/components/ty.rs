use dominator::Dom;

#[derive(Default, Clone, Debug)]
pub enum Colour {
    Grey,
    Blue,
    Red,
    #[default]
    Pink,
    Orange,
    Hex(String)
}

#[derive(Default, Clone, Debug)]
pub enum RemSizing {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Rem(f32),
}

pub trait Component {
    fn render(self) -> Dom;
}