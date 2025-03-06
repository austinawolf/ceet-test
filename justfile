
test-lib:
    mkdir -p out_dir
    gcc -fPIC -c c/rain.c -o out_dir/ctest.o -g -Ic/
    gcc -fPIC -c tests/test_sensor.c -o out_dir/test_sensor.o -g -Ic/
    gcc -shared -o libtest.so out_dir/test_sensor.o out_dir/ctest.o

build:
    cargo build

run:
    cargo run
