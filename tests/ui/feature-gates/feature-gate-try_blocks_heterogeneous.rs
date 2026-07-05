//@ edition: 2018

pub fn main() {
    let try_result = try Option<_> { //~ ERROR `try` expression is experimental
        let x = 5;
        x
    };
    assert_eq!(try_result, Some(5));

    // The heterogenous form is new, so is gated even under a `cfg(false)`.
    // See <https://github.com/rust-lang/rust/issues/152501>

    #[cfg(false)]
    try () {}
    //~^ error `try` expression is experimental
}
