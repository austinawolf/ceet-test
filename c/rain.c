#include "rain.h"
#include <setjmp.h>

static jmp_buf env;

static rain_test_results_t *_results = NULL;

void rain_test_assert(bool condition, const char *file, int lineno)
{
    if (condition)
    {
        return;
    }

    _results->file = file;
    _results->lineno = lineno;

    longjmp(env, 1);
}

int rain_test_run(rain_test_fut_t fut, rain_test_results_t *results)
{
    // check magic

    // initialize results
    _results = results;
    results->assert = false;

    int ret = setjmp(env);
    if (ret != 0)
    {
        results->assert = true;
        return results->magic;
    }

    fut();

    return results->magic;
}