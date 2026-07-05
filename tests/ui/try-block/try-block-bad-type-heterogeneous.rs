//@ edition: 2018

#![feature(try_blocks_heterogeneous)]

pub fn main() {
    let res = try Result<u32, std::array::TryFromSliceError> {
        Err("")?; //~ ERROR `?` couldn't convert the error
        5
    };

    let res = try Result<i32, i32> {
        "" //~ ERROR type mismatch
    };

    let res = try Result<i32, i32> { }; //~ ERROR type mismatch

    let res = try () { };
    //~^ ERROR a `try` block must return `Result` or `Option`

    let res = try i32 { 5 }; //~ ERROR a `try` block must return `Result` or `Option`
}
