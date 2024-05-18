mod models;

pub mod prelude {
    pub use crate::models::*;

    pub(crate) use bar::prelude::*;
    pub(crate) use e::prelude::*;
    pub(crate) use f::prelude::*;
    pub(crate) use foo::prelude::*;
    pub(crate) use o::prelude::*;
}

pub use prelude::*;

uniffi::include_scaffolding!("foobar");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(Foobar::default(), Foobar::default(),);
    }
}
