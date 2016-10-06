When mutable data is borrowed, its owner *freezes*. Borrowed data can't be
modified via the original owner until all references to it go out of scope,
however it can be modified by a mutable reference, using the dereference
operator:

{freeze.play}
