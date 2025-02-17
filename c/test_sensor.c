#include <stdio.h>
#include <setjmp.h>
#include <stdbool.h>

static jmp_buf _test_env;

static int _counter = 0;


static void _setup(void)
{
    int ret = setjmp(_test_env);

    if (ret != 0)
    {
        printf("Test failed\n");
    } 
}

static void _assert(bool cond)
{
    if (cond) 
    {
        return;
    }

    printf("ASSERT\n");
    longjmp(_test_env, 1);
}

void test_asdf(void) 
{
    _setup();
    
    printf("running test_asdf\n");
    printf("counter: %d\n", _counter++);

    _assert(true);

}

void test_fdas(void) 
{
    _setup();
    printf("running test_fdas\n");
    printf("counter: %d\n", _counter++);

    _assert(false);
}
