#include "rain.h"
#include <setjmp.h>

static jmp_buf env;


void ctest_test_runner(ctype_test_function_t f)
{
    printf("Starting test...\n");

    int ret = setjmp(env);
    if (ret != 0)
    {
        printf("Got assert\n");
        return;
    }

    f();

    printf("Test complete\n");

    return;
}

void ctest_assert(bool condition)
{
    if (condition)
    {
        return;
    }

    printf("ASSERT\n");
    longjmp(env, 1);
}
