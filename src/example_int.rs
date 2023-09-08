mod rave;

fn main() {
    // Alloc
    let mut v = 0;
    let adr = Alloc::i32(21).unwrap();

    Read::int(adr, &mut v);
    println!("{}", v);

    // Dealloc & Delete -> variables
    Dealloc::int(adr);
    Delete::int(&mut v);
}
