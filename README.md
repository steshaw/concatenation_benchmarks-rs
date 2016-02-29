#Comparing ways to concatenate strings in Rusts

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here are benchmarks that show what is slow and fast.

## Contribution

I'd be happy to add more benchmarks if you send me PRs with other ways to do it.

## Results on my machine

```

     Running target/release/concatenation_benchmarks-0b60c7ba31bdb626

running 5 tests
test format_macro        ... bench:         112 ns/iter (+/- 13)
test mut_string_push_str ... bench:          44 ns/iter (+/- 3)
test string_from_plus_op ... bench:          44 ns/iter (+/- 3)
test to_owned_plus_op    ... bench:          59 ns/iter (+/- 3)
test to_string_plus_op   ... bench:          86 ns/iter (+/- 5)

test result: ok. 0 passed; 0 failed; 0 ignored; 5 measured

```
