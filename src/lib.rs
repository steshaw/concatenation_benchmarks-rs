#![feature(test)]
extern crate test;

static DATE: &'static str = "2014-11-28";
static TIME: &'static str = "12:00:09Z";


use test::Bencher;

#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(||{
        let datetime = format!("{}T{}", DATE, TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime = &(DATE.to_string() + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime = &(DATE.to_owned() + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime = &(String::from(DATE) + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn mut_string_push_str(b: &mut Bencher) {
    b.iter(||{
        let mut datetime = String::new();
        datetime.push_str(DATE);
        datetime.push_str("T");
        datetime.push_str(TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn array_concat(b: &mut Bencher) {
    b.iter(||{
        let datetime = [ DATE, "T", TIME ].concat();
        test::black_box(datetime);
    });
}

#[bench]
fn array_join(b: &mut Bencher) {
    b.iter(||{
        let datetime = [ "2014-11-28", "12:00:09Z" ].join("T");
        test::black_box(datetime);
    });
}
