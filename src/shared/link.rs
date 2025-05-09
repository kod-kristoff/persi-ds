pub trait Link: Sized {
    type ValueType;

    fn from_value(element: Self::ValueType) -> Self;
    fn cons(element: Self::ValueType, next: Option<Self>) -> Self;
    fn clone(&self) -> Self;
    fn get_element(&self) -> &Self::ValueType;
    // fn next(&self) -> Option<&Self>;
    fn next_cloned(&self) -> Option<Self>;
    fn next_ref(&self) -> Option<&Self>;
    fn link_ref(&self) -> &Self;
}

// impl Debug for Link
// where
//     Link: Debug,
// {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {}
// }
