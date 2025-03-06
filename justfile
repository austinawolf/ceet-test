
test-lib:
    mkdir -p out_dir
    gcc -fPIC -c src/c/rain.c -o out_dir/ctest.o -g -Isrc/c/
    gcc -fPIC -c tests/test_sensor.c -o out_dir/test_sensor.o -g -Isrc/c/
    gcc -shared -o libtest.so out_dir/test_sensor.o out_dir/ctest.o

build:
    cargo build

run:
    cargo run
