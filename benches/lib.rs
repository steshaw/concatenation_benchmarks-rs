#![feature(test)]
extern crate test;
extern crate criterion;

static DATE: &'static str = "2014-11-28";
static T: &'static str = "T";
static TIME: &'static str = "12:00:09Z";
static DATETIME: &'static str = "2014-11-28T12:00:09Z";


use criterion::{Bencher, Criterion};

fn collect_to_string(b: &mut Bencher) {
    let list = vec![DATE, T, TIME];
    b.iter(|| {
               let datetime: String = list.iter().map(|x| *x).collect();
               test::black_box(datetime);
           });
}

#[test]
fn criterion_collect_to_string() {
    Criterion::default()
        .bench_function("collect to string", collect_to_string);
}

#[test]
fn criterion_format_macro() {
    Criterion::default()
        .bench_function("format_macro", format_macro);
}

fn format_macro(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &format!("{}T{}", DATE, TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_to_string_plus_op() {
    Criterion::default()
        .bench_function("to_string_plus_op", to_string_plus_op);
}

fn to_string_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(DATE.to_string() + "T" + TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_to_owned_plus_op() {
    Criterion::default()
        .bench_function("to_owned_plus_op", to_owned_plus_op);
}

fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(DATE.to_owned() + "T" + TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_string_from_plus_op() {
    Criterion::default()
        .bench_function("string_from_plus_op", string_from_plus_op);
}

fn string_from_plus_op(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(String::from(DATE) + "T" + TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_string_from_all() {
    Criterion::default()
        .bench_function("string_from_all", string_from_all);
}

fn string_from_all(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &(String::from(DATE) + &String::from("T") + &String::from(TIME));
               test::black_box(datetime);
           });
}

#[test]
fn criterion_write_macro() {
    Criterion::default()
        .bench_function("write_macro", write_macro);
}

fn write_macro(b: &mut Bencher) {
    b.iter(|| {
               use std::io::Write;
               let mut datetime = Vec::new();
               write!(&mut datetime, "{}T{}", DATE, TIME).unwrap();
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_with_capacity_push_str() {
    Criterion::default()
        .bench_function("mut_string_with_capacity_push_str", mut_string_with_capacity_push_str);
}

fn mut_string_with_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(20);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_with_capacity_push_str_char() {
    Criterion::default()
        .bench_function("mut_string_with_capacity_push_str_char", mut_string_with_capacity_push_str_char);
}

fn mut_string_with_capacity_push_str_char(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(20);
               datetime.push_str(DATE);
               datetime.push('T');
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_with_too_little_capacity_push_str() {
    Criterion::default()
        .bench_function("mut_string_with_too_little_capacity_push_str", mut_string_with_too_little_capacity_push_str);
}

fn mut_string_with_too_little_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(2);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_with_too_much_capacity_push_str() {
    Criterion::default()
        .bench_function("mut_string_with_too_much_capacity_push_str", mut_string_with_too_much_capacity_push_str);
}

fn mut_string_with_too_much_capacity_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::with_capacity(200);
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_push_string() {
    Criterion::default()
        .bench_function("mut_string_push_string", mut_string_push_string);
}

fn mut_string_push_string(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = Vec::<String>::new();
               datetime.push(String::from(DATE));
               datetime.push(String::from("T"));
               datetime.push(String::from(TIME));
               let datetime = datetime.join("");
               test::black_box(datetime);
           });
}

#[test]
fn criterion_mut_string_push_str() {
    Criterion::default()
        .bench_function("mut_string_push_str", mut_string_push_str);
}

fn mut_string_push_str(b: &mut Bencher) {
    b.iter(|| {
               let mut datetime = String::new();
               datetime.push_str(DATE);
               datetime.push_str("T");
               datetime.push_str(TIME);
               test::black_box(datetime);
           });
}

#[test]
fn criterion_array_concat() {
    Criterion::default()
        .bench_function("array_concat", array_concat);
}

fn array_concat(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, "T", TIME].concat();
               test::black_box(datetime);
           });
}

#[test]
fn criterion_array_join() {
    Criterion::default()
        .bench_function("array_join", array_join);
}

fn array_join(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, TIME].join("T");
               test::black_box(datetime);
           });
}

#[test]
fn criterion_array_join_long() {
    Criterion::default()
        .bench_function("array_join_long", array_join_long);
}

fn array_join_long(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, "T", TIME].join("");
               test::black_box(datetime);
           });
}

#[test]
fn criterion_array_join_empty_arg() {
    Criterion::default()
        .bench_function("array_join_empty_arg", array_join_empty_arg);
}

fn array_join_empty_arg(b: &mut Bencher) {
    b.iter(|| {
               let datetime: &str = &[DATE, "T", TIME].join("");
               test::black_box(datetime);
           });
}

#[cfg(unix)]
#[test]
fn criterion_from_bytes() {
    Criterion::default()
        .bench_function("from_bytes", from_bytes);
}

#[cfg(unix)]
fn from_bytes(b: &mut Bencher) {
    use std::slice;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    b.iter(|| {
               let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

               let datetime = OsStr::from_bytes(bytes);
               test::black_box(datetime);
           });
}
