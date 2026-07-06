// rustfmt-edition: 2018
#![feature(try_blocks_heterogeneous)]

fn main() -> Result<(), !> {
    let _x = try Option<_> { 4 };

    try Result<_, _> {}
}

fn baz() -> Option<i32> {
    if (1 == 1) {
        return try Option<i32> { 5 };
    }

    // test
    let x = try Option<()> {
        // try blocks are great
    };

    let y = try Option<i32> { 6 }; // comment

    let x = try /* Invisible comment */ Option<()> {};
    let x = try Option<()> /* Invisible comment */ {};

    let x = try Option<i32> {
        baz()?;
        baz()?;
        baz()?;
        7
    };

    let x = try Foo<Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar> {
        1 + 1 + 1
    };

    let x =
        try Foo<Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar, Bar> {};

    let x = try Result<
        VeryVeryVeryVeryVeryLongTypeForSuccess,
        VeryVeryVeryVeryVeryLongTypeForFailure,
    > {
    };

    let _ = overflowed_expr(x, try Option<_> {
        foo()?;
        bar()?;
    });

    return None;
}
