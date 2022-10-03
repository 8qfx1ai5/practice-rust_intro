#[path = "subfolder/file2.rs"] mod file2;

fn main() {
    println!("Hello World!");
    file2::greetings();
}
