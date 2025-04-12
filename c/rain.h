#ifndef CTEST_H
#define CTEST_H

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>



#define _CONCAT(a, b) a##b
#define CONCAT(a, b) _CONCAT(a, b)
#define TEST_HOOK_PREFIX    _rain_test_hook__


#define TEST(__function_name__)             \
void CONCAT(CONCAT(CONCAT(TEST_HOOK_PREFIX, __LINE__), __), __function_name__)(void)

#define ASSERT(__condition__)   rain_test_assert(__condition__, __FILE__, __LINE__)

typedef struct
{
    int32_t magic;
    bool assert;
    const char *file;
    int32_t lineno;
} rain_test_results_t;

typedef void (*rain_test_fut_t)(void);

void rain_test_assert(bool condition, const char *file, int lineno);

int rain_test_run(rain_test_fut_t fut, rain_test_results_t *results);

#endif
