# Surgery
Inverse versions of both the `head` and `tail` coreutils


## Decapitate

The `dcap` binary is the inverse of the `head` coreutil.
Where `head` prints the first N lines and skips the rest, decapitate will skip the first N lines and print the rest


## Amputate

The `amp` binary is the inverse of the `tail` coreutil.
Where `tail` prints the last N lines and skips everything before, amputate will print everything before and skip the last N lines.



## Building 
A simple `cargo build` should do :-)
Run the tests with `cargo test`
