use cursive::view::View;
use std::any::Any;

/// Represents a type that can be made into a `Box<View>`.
pub trait IntoBoxedView {
    /// Returns a `Box<View>`.
    fn as_boxed_view(self) -> Box<dyn ViewAndTest>;
}

impl<T> IntoBoxedView for T
where
    T: ViewAndTest,
{
    fn as_boxed_view(self) -> Box<dyn ViewAndTest> {
        Box::new(self)
    }
}

impl IntoBoxedView for Box<dyn ViewAndTest> {
    fn as_boxed_view(self) -> Box<dyn ViewAndTest> {
        self
    }
}

pub trait Test: Any {
    fn get_three(self) -> usize;
}

pub trait ViewAndTest: View + Test {
    //fn get_three(self) -> usize;
}
//impl<T> ViewAndTest for T where T: View + Test {}
