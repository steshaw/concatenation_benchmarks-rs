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

running 19 tests
test array_concat                                 ... bench:          44 ns/iter (+/- 0)
test array_connect                                ... bench:          44 ns/iter (+/- 1)
test array_join                                   ... bench:          44 ns/iter (+/- 1)
test array_join_empty_arg                         ... bench:          44 ns/iter (+/- 3)
test array_join_long                              ... bench:          46 ns/iter (+/- 7)
test format_macro                                 ... bench:         116 ns/iter (+/- 2)
test mut_string_push_str                          ... bench:          53 ns/iter (+/- 1)
test mut_string_push_string                       ... bench:         120 ns/iter (+/- 2)
test mut_string_with_capacity_push_str            ... bench:          19 ns/iter (+/- 1)
test mut_string_with_capacity_push_str_char       ... bench:          19 ns/iter (+/- 0)
test mut_string_with_too_little_capacity_push_str ... bench:          84 ns/iter (+/- 5)
test mut_string_with_too_much_capacity_push_str   ... bench:          19 ns/iter (+/- 0)
test string_from_all                              ... bench:          50 ns/iter (+/- 2)
test string_from_plus_op                          ... bench:          55 ns/iter (+/- 7)
test to_owned_plus_op                             ... bench:          62 ns/iter (+/- 1)
test to_string_plus_op                            ... bench:          94 ns/iter (+/- 9)
test very_unsafe                                  ... bench:          55 ns/iter (+/- 1)
test very_unsafe_no_convert                       ... bench:          31 ns/iter (+/- 0)
test write_macro                                  ... bench:         121 ns/iter (+/- 22)

test result: ok. 0 passed; 0 failed; 0 ignored; 19 measured
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

~49ns

### array `.join()`

```rust
let datetime = [ DATE, TIME ].join("T");
```

~51ns

Or with an empty `str` as argument

```rust
let datetime:&str = &[ DATE, "T", TIME ].join("");
```

~49ns

### `format!` macro


```rust
let datetime = format!("{}T{}", DATE, TIME);
```

~115ns

### `push_str()` into `mut String`

```rust
let mut datetime = String::new();
        datetime.push_str(DATE);
        datetime.push_str("T");
        datetime.push_str(TIME);
```

~50ns

### `push()` into `mut Vec<String>`

```rust
let mut datetime = Vec::<String>::new();
datetime.push(String::from(DATE));
datetime.push(String::from("T"));
datetime.push(String::from(TIME));
let datetime = datetime.join("");
```

Nope, this one is really expensive: ~121ns

### `push_str()` into `mut String::with_capacity()`

```rust
let mut datetime = String::with_capacity(20);
datetime.push_str(DATE);
datetime.push_str("T");
datetime.push_str(TIME);
```

Ladies and Gentlemen, we have a winner!

~19ns

### `String::from()` and +operator

```rust
let datetime = &(String::from(DATE) + "T" + TIME);
```

~51ns

### `.to_string()` and +operator


```rust
let datetime = &(DATE.to_string() + "T" + TIME);
```

~88ns

### `.to_owned()` and +operator

```rust
let datetime = &(DATE.to_owned() + "T" + TIME);
```

## Contribution

I'd be happy to add more benchmarks if you send me PRs with other ways to do it.



