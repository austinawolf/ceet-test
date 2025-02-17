
test-lib:
    gcc -fPIC -c c/test_sensor.c -o out_dir/mylib.o -g
    gcc -shared -o libtest.so out_dir/mylib.o


build:
    cargo build

run:
    cargo run
