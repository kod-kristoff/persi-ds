use core::borrow::Borrow;

pub trait RBNode: Sized {
    type ValueType;

    /// The colour of a leaf must be `Colour::Red`.
    fn leaf(element: Self::ValueType) -> Self;
    fn clone(&self) -> Self;
    fn get_element(&self) -> &Self::ValueType;
    fn get_colour(&self) -> Colour;
    fn left_cloned(&self) -> Option<Self>;
    fn right_cloned(&self) -> Option<Self>;
    fn contains<Q>(&self, q: &Q) -> bool
    where
        Self::ValueType: Borrow<Q> + PartialOrd,
        Q: PartialOrd + ?Sized;
    fn get<Q>(&self, q: &Q) -> Option<&Self::ValueType>
    where
        Self::ValueType: Borrow<Q> + PartialOrd,
        Q: PartialOrd + ?Sized;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Red,
    Black,
}
