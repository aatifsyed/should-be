//! Postfix assertions of equality
//! ```
//! use should_be::ShouldBe; // Import the trait
//! let yaks_shaved = 0;
//! yaks_shaved.should_be(0); // passes
//! ```
//! ```should_panic
//! # use should_be::ShouldBe;
//! # let yaks_shaved = 0;
//! yaks_shaved.should_be(1); // panics
//! // you can provide a panic message using a tuple.
//! yaks_shaved.should_be((1, "why haven't you shaved?"))
//! ```

pub trait ShouldBe: PartialEq + std::fmt::Debug + Sized {
    /// # Panics
    /// - If `expected != self`.
    fn should_be(&self, expected: impl ShouldBeArgs<Self>) {
        match expected.message() {
            None => assert_eq!(expected.expected(), self),
            Some(message) => assert_eq!(expected.expected(), self, "{}", message),
        }
    }
}

impl<T> ShouldBe for T where T: PartialEq + std::fmt::Debug + Sized {}

pub trait ShouldBeArgs<T> {
    type MessageT: std::fmt::Display;
    fn expected(&self) -> &T;
    fn message(&self) -> Option<&Self::MessageT> {
        None
    }
}

impl<T> ShouldBeArgs<T> for T {
    type MessageT = &'static str;
    fn expected(&self) -> &T {
        &self
    }
}

impl<T> ShouldBeArgs<T> for &T {
    type MessageT = &'static str;
    fn expected(&self) -> &T {
        self
    }
}

impl<T, MessageT> ShouldBeArgs<T> for (T, MessageT)
where
    MessageT: std::fmt::Display,
{
    type MessageT = MessageT;
    fn expected(&self) -> &T {
        &self.0
    }

    fn message(&self) -> Option<&Self::MessageT> {
        Some(&self.1)
    }
}

impl<T, MessageT> ShouldBeArgs<T> for (&T, MessageT)
where
    MessageT: std::fmt::Display,
{
    type MessageT = MessageT;
    fn expected(&self) -> &T {
        self.0
    }

    fn message(&self) -> Option<&Self::MessageT> {
        Some(&self.1)
    }
}
