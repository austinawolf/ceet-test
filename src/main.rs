use std::env;


extern "C" {
    fn sensor_init();
    fn sensor_get_value() -> i32;
}

fn main() {
    env::set_var("OUT_DIR", "./out_dir");
    env::set_var("TARGET", "x86_64-unknown-linux-gnu");
    env::set_var("HOST", "x86_64-unknown-linux-gnu");
    env::set_var("OPT_LEVEL", "0");

    println!("Hello, world1!");

    cc::Build::new()
    .file("c/test_sensor.c")
    .flag("-g")
    .compile("test");
}
