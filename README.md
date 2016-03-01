#Comparing ways to concatenate strings in Rusts

There are many ways to turn a `&str` into a `String` in Rust and therefore many ways to concatenate two `&str`s.

Here are benchmarks that show what is slow and fast.

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

Interestingly, this is as slow as `concat()` in the presence of another benchmark with join or connect.
Check out the commented benchmarks.

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

