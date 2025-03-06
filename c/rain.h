#ifndef CTEST_H
#define CTEST_H

#include <stdio.h>
#include <stdbool.h>


#define _CONCAT(a, b) a##b
#define CONCAT(a, b) _CONCAT(a, b)

#define TEST_HOOK_PREFIX    _ctest_hook__
#define FUT(__f__, __ln__)   void CONCAT(CONCAT(TEST_HOOK_PREFIX, __ln__), __ ## __f__)(void)
#define VOID_HEADER(__g__)      void __g__(void)

#define TEST(__function_name__)             \
VOID_HEADER(__function_name__);             \
FUT(__function_name__, __LINE__ );          \
FUT(__function_name__, __LINE__ )           \
{                                           \
    ctest_test_runner(__function_name__);   \
}                                           \
void __function_name__(void)                \



typedef void (*ctype_test_function_t)(void);


void ctest_test_runner(ctype_test_function_t f);

void ctest_assert(bool condition);

#endif
