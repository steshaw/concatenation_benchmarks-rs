#![feature(test)]
extern crate test;

static DATE: &'static str = "2014-11-28";
static TIME: &'static str = "12:00:09Z";


use test::Bencher;

#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(||{
        let _datetime = format!("{}T{}", DATE, TIME);
    });
}

#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(||{
        let _datetime = &(DATE.to_string() + "T" + TIME);
    });
}

#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(||{
        let _datetime = &(DATE.to_owned() + "T" + TIME);
    });
}

#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(||{
        let _datetime = &(String::from(DATE) + "T" + TIME);
    });
}

#[bench]
fn mut_string_push_str(b: &mut Bencher) {
    b.iter(||{
        let mut _datetime = String::new();
        _datetime.push_str(DATE);
        _datetime.push_str("T");
        _datetime.push_str(TIME);
    });
}
