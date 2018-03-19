# Comparing ways to concatenate strings in Rust

## DISCLAIMER

These numbers are already a bit dated - **your results may vary** - so please rerun these numbers yourself!

## Intro

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here are benchmarks that show what is slow and fast.
Thanks to all the comments on and discussion on [reddit](https://www.reddit.com/r/rust/comments/48fmta/seven_ways_to_concatenate_strings_in_rust_the/) where I posted these originally only 7 benchmarks. Some go into the details of what is going on in the background of these operations.

## Results on my machine

```
$ cargo bench

running 18 tests
test array_concat                                 ... bench:          34 ns/iter (+/- 20)
test array_join                                   ... bench:          36 ns/iter (+/- 8)
test array_join_empty_arg                         ... bench:          34 ns/iter (+/- 20)
test array_join_long                              ... bench:          33 ns/iter (+/- 17)
test collect_to_string                            ... bench:          75 ns/iter (+/- 37)
test format_macro                                 ... bench:         106 ns/iter (+/- 56)
test from_bytes                                   ... bench:           1 ns/iter (+/- 0)
test mut_string_push_str                          ... bench:          67 ns/iter (+/- 22)
test mut_string_push_string                       ... bench:         170 ns/iter (+/- 38)
test mut_string_with_capacity_push_str            ... bench:          30 ns/iter (+/- 22)
test mut_string_with_capacity_push_str_char       ... bench:          25 ns/iter (+/- 15)
test mut_string_with_too_little_capacity_push_str ... bench:          93 ns/iter (+/- 16)
test mut_string_with_too_much_capacity_push_str   ... bench:          30 ns/iter (+/- 14)
test string_from_all                              ... bench:         130 ns/iter (+/- 62)
test string_from_plus_op                          ... bench:          72 ns/iter (+/- 42)
test to_owned_plus_op                             ... bench:          72 ns/iter (+/- 45)
test to_string_plus_op                            ... bench:          72 ns/iter (+/- 18)
test write_macro                                  ... bench:         109 ns/iter (+/- 60)

test result: ok. 0 passed; 0 failed; 0 ignored; 18 measured; 0 filtered out

```

Thanks also to @llogiq for posting his [numbers](https://github.com/hoodie/concatenation_benchmarks-rs/pull/2#issuecomment-192680412)

## Examples so far


```rust
let DATE = "2014-11-28";
let TIME = "12:00:09Z";

// wanting to create

let datetime = "2014-11-28T12:00:09Z";

```


### array `.concat()`

```rust
let datetime = [ DATE, "T", TIME ].concat();
```



### array `.join()`

```rust
let datetime = [ DATE, TIME ].join("T");
```



Or with an empty `str` as argument

```rust
let datetime:&str = &[ DATE, "T", TIME ].join("");
```



### `format!` macro


```rust
let datetime = format!("{}T{}", DATE, TIME);
```



### `push_str()` into `mut String`

```rust
let mut datetime = String::new();
        datetime.push_str(DATE);
        datetime.push_str("T");
        datetime.push_str(TIME);
```



### `push()` into `mut Vec<String>`

```rust
let mut datetime = Vec::<String>::new();
datetime.push(String::from(DATE));
datetime.push(String::from("T"));
datetime.push(String::from(TIME));
let datetime = datetime.join("");
```



### `push_str()` into `mut String::with_capacity()`

```rust
let mut datetime = String::with_capacity(20);
datetime.push_str(DATE);
datetime.push_str("T");
datetime.push_str(TIME);
```

Ladies and Gentlemen, we have a winner!



### `String::from()` and +operator

```rust
let datetime = &(String::from(DATE) + "T" + TIME);
```



### `.to_string()` and +operator


```rust
let datetime = &(DATE.to_string() + "T" + TIME);
```



### `.to_owned()` and +operator

```rust
let datetime = &(DATE.to_owned() + "T" + TIME);
```

## Contribution

I'd be happy to add more benchmarks if you send me PRs with other ways to do it.



