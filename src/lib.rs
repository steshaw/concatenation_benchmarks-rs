#![feature(test)]
extern crate test;

static DATE: &'static str = "2014-11-28";
static TIME: &'static str = "12:00:09Z";


use test::Bencher;

#[bench]
fn format_macro(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &format!("{}T{}", DATE, TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn to_string_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &(DATE.to_string() + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn to_owned_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &(DATE.to_owned() + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn string_from_plus_op(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &(String::from(DATE) + "T" + TIME);
        test::black_box(datetime);
    });
}

#[bench]
fn string_from_all(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &(String::from(DATE) + &String::from("T") + &String::from(TIME));
        test::black_box(datetime);
    });
}

#[bench]
fn write_macro(b: &mut Bencher) {
    b.iter(||{
        use std::io::Write;
        let mut datetime = Vec::new();
        write!(&mut datetime, "{}T{}", DATE,TIME);

        test::black_box(datetime);
    });
}

#[bench]
fn mut_string_push_string(b: &mut Bencher) {
    b.iter(||{
        let mut datetime = Vec::<String>::new();
        datetime.push(String::from(DATE));
        datetime.push(String::from("T"));
        datetime.push(String::from(TIME));
        let datetime = datetime.join("");
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
        let datetime:&str = &[ DATE, "T", TIME ].concat();
        test::black_box(datetime);
    });
}

#[bench]
fn array_join(b: &mut Bencher) {
    b.iter(||{
        let datetime:&str = &[ DATE, TIME ].join("T");
        test::black_box(datetime);
    });
}

// funny, the presence of this benchmark makes the join() version slower
//#[bench]
//fn array_connect(b: &mut Bencher) {
//    b.iter(||{
//        let datetime:&str = &[ DATE, TIME ].connect("T");
//        test::black_box(datetime);
//    });
//}

// adding this one also makes array_join() slower
//#[bench]
//fn array_join_long(b: &mut Bencher) {
//    b.iter(||{
//        let datetime:&str = &[ DATE, "T", TIME ].join("");
//        test::black_box(datetime);
//    });
//}

