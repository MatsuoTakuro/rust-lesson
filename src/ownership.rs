pub fn run() {
    let s1 = String::from("hello");
    println!("Heap  address of s1 is: {:?}", s1.as_ptr());
    let s2 =s1;
    println!("{}", s2);
    println!("Heap  address of s2 is: {:?}", s2.as_ptr());

    println!("\n******\n");

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    println!("\n******\n");

    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    println!("\n******\n");

    let s3 = String::from("hello");
    let s4 =s3.clone();
    println!("{} {}", s3, s4);
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap  address of s3 is: {:?}", s3.as_ptr());
    println!("Heap  address of s4 is: {:?}", s4.as_ptr());

    println!("\n******\n");

    let s5 = String::from("hello");
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap  address of s5 is: {:?}", s5.as_ptr());
    println!("Len           of s5 is: {}", s5.len());
    println!("Capacity      of s5 is: {}", s5.capacity());
    take_ownership(s5);
    // println!("{}", s5);
    println!("\n******\n");

    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap  address of s6 is: {:?}", s6.as_ptr());
    println!("Len           of s6 is: {}", s6.len());
    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap  address of s7 is: {:?}", s7.as_ptr());
    println!("Len           of s7 is: {}", s7.len());

    println!("\n******\n");

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is: {}", s8, len);

    println!("\n******\n");

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    println!("\n******\n");

    let s10 = String::from("hello");
    let ref1 = &s10;
    let ref2 = &s10;
    println!("{} {} {}", s10, ref1, ref2);

    println!("\n******\n");

    // let mut s10 = String::from("hello");
    // let ref1 = &s10;
    // let ref2 = &mut s10;
    // println!("{} {}", ref1, ref2);
    let mut s11 = String::from("hello");
    let ref1 = &mut s11;
    println!("{}", ref1);
    println!("{}", s11);

    println!("\n******\n");

    let mut s12 = String::from("hello");
    let ref1 = &s12;
    let ref2 = &s12;
    println!("{} {}", ref1, ref2);
    let ref3 = &mut s12;
    *ref3 = String::from("hello_updated");
    println!("{}", s12);

    println!("\n*** Finished ***\n");
}

fn take_ownership(s: String) {
    println!("Stack address of s  is: {:p}", &s);
    println!("Heap  address of s  is: {:?}", s.as_ptr());
    println!("Len           of s  is: {}", s.len());
    println!("Capacity      of s  is: {}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world")
}
