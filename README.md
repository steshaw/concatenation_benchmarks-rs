#Comparing ways to concatenate strings in rust

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here are benchmarks that show what is slow and fast.

## Results on my machine

```
$ cargo bench

running 12 tests
test array_concat                      ... bench:          48 ns/iter (+/- 14)
test array_join                        ... bench:          51 ns/iter (+/- 4)
test array_join_empty_arg              ... bench:          49 ns/iter (+/- 3)
test format_macro                      ... bench:         115 ns/iter (+/- 8)
test mut_string_push_str               ... bench:          50 ns/iter (+/- 9)
test mut_string_push_string            ... bench:         121 ns/iter (+/- 5)
test mut_string_with_capacity_push_str ... bench:          19 ns/iter (+/- 2)
test string_from_all                   ... bench:          48 ns/iter (+/- 8)
test string_from_plus_op               ... bench:          51 ns/iter (+/- 6)
test to_owned_plus_op                  ... bench:          58 ns/iter (+/- 4)
test to_string_plus_op                 ... bench:          88 ns/iter (+/- 3)
test write_macro                       ... bench:         117 ns/iter (+/- 54)

test result: ok. 0 passed; 0 failed; 0 ignored; 12 measured
```

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

