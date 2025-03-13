#ifndef CTEST_H
#define CTEST_H

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>



#define _CONCAT(a, b) a##b
#define CONCAT(a, b) _CONCAT(a, b)
#define TEST_HOOK_PREFIX    _ctest_hook__


#define TEST(__function_name__)             \
void CONCAT(TEST_HOOK_PREFIX, __function_name__)(void)                \

typedef struct
{
    int32_t magic;
    bool assert;
} rain_test_results_t;

typedef void (*rain_test_fut_t)(void);

void rain_test_assert(bool condition);

int rain_test_run(rain_test_fut_t fut, rain_test_results_t *results);

#endif
