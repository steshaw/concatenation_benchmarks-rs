#![feature(test)]

extern crate test;

static DATE: &str = "2014-11-28";
static T: &str = "T";
static TIME: &str = "12:00:09Z";
static DATETIME: &str = "2014-11-28T12:00:09Z";


#[test]
fn from_bytes() {
    use std::slice;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let bytes = unsafe { slice::from_raw_parts(DATE.as_ptr(), 20) };

    let datetime = OsStr::from_bytes(bytes);

    assert_eq!(String::from(DATETIME),
               datetime.to_owned().into_string().unwrap());
}

#[test]
fn format_macro() {
    let datetime: &str = &format!("{}{}{}", DATE, T, TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn to_string_plus_op() {
    let datetime: &str = &(DATE.to_string() + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn to_owned_plus_op() {
    let datetime: &str = &(DATE.to_owned() + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn string_from_plus_op() {
    let datetime: &str = &(String::from(DATE) + T + TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn string_from_all() {
    let datetime: &str = &(String::from(DATE) + &String::from("T") + &String::from(TIME));
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn mut_string_with_capacity_push_str() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn mut_string_with_capacity_push_str_char() {
    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push('T');
}

#[test]
fn mut_string_with_too_little_capacity_push_str() {
    let mut datetime = String::with_capacity(2);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn mut_string_with_too_much_capacity_push_str() {
    let mut datetime = String::with_capacity(200);
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn mut_string_push_string() {
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(DATE));
    datetime.push(String::from(T));
    datetime.push(String::from(TIME));
    let datetime = datetime.join("");
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn mut_string_push_str() {
    let mut datetime = String::new();
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn array_concat() {
    let datetime: &str = &[DATE, T, TIME].concat();
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn array_join() {
    let datetime: &str = &[DATE, TIME].join(T);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn array_connect() {
    let datetime: &str = &[DATE, TIME].join(T);
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn array_join_long() {
    let datetime: &str = &[DATE, "T", TIME].join("");
    assert_eq!(String::from(DATETIME), datetime);
}

#[test]
fn array_join_empty_arg() {
    let datetime: &str = &[DATE, "T", TIME].join();
    assert_eq!(String::from(DATETIME), datetime);
}
