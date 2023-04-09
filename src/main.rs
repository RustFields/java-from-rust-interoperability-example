mod jnilib;

fn main() {
    println!("{}", jnilib::abs(-10).expect("ERROR"));
}
