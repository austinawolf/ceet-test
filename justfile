
test-lib:
    gcc -fPIC -c src/ctest/ctest.c -o out_dir/ctest.o -g -Isrc/ctest/
    gcc -fPIC -c c/test_sensor.c -o out_dir/test_sensor.o -g -Isrc/ctest/
    gcc -shared -o libtest.so out_dir/test_sensor.o out_dir/ctest.o


build:
    cargo build

run:
    cargo run
