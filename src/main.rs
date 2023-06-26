fn main() {
    let args: Vec<_> = std::env::args().collect();
    let program = &args[1..].join(" ");
    let command = std::ffi::CString::new(program.to_owned()).unwrap();

    let start_time = std::time::Instant::now();
    unsafe { libc::system(command.as_ptr()) };
    let end_time = std::time::Instant::now();

    let diff = end_time - start_time;
    println!("It took {:?}", diff);
}
