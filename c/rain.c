#include "rain.h"
#include <setjmp.h>

static jmp_buf env;


void rain_test_assert(bool condition)
{
    if (condition)
    {
        return;
    }

    printf("ASSERT\n");
    longjmp(env, 1);
}

int rain_test_run(rain_test_fut_t fut, rain_test_results_t *results)
{
    // check magic

    // initialize results
    results->assert = false;

    int ret = setjmp(env);
    if (ret != 0)
    {
        results->assert = true;
        return results->magic;
    }

    fut();

    printf("Test complete\n");

    return results->magic;
}