#Comparing ways to concatenate strings in Rusts

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here are benchmarks that show what is slow and fast.

## Contribution

I'd be happy to add more benchmarks if you send me PRs with other ways to do it.

## Results on my machine

```
$ cargo bench

running 7 tests
test array_concat        ... bench:          51 ns/iter (+/- 3)
test array_join          ... bench:          19 ns/iter (+/- 0)
test format_macro        ... bench:         114 ns/iter (+/- 3)
test mut_string_push_str ... bench:          48 ns/iter (+/- 2)
test string_from_plus_op ... bench:          48 ns/iter (+/- 3)
test to_owned_plus_op    ... bench:          58 ns/iter (+/- 3)
test to_string_plus_op   ... bench:          86 ns/iter (+/- 3)

test result: ok. 0 passed; 0 failed; 0 ignored; 7 measured
```
