fn main() {
    let mut _mutable_integer = 7_i32;

    {
        // Borrow `_mutable_integer`
        let _large_integer = &_mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50_i32;
        // FIXME ^ Comment out this line

        // `_large_integer` goes out of scope
    }

    {
        // Mutable Borrow `_mutable_integer`
        let _large_integer = &mut _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50_i32;
        // FIXME ^ Comment out this line

        // Ok! Using dereferencing
        *_large_integer = 6_i32;

        // `_large_integer` goes out of scope
    }

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3_i32;
}
