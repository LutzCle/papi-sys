// Copyright 2019 Yeonsoo Kim
// Author: Yeonsoo Kim <alkorang@outlook.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#include <papi.h>
#include <stdio.h>

#define CODEGEN_DEFINE(var) codegen_define(#var, var)

void codegen_define(const char *name, const int val) {
    printf("pub const %s: ::std::os::raw::c_int = %d;\n", name, val);
}

void codegen() {
    CODEGEN_DEFINE(PAPI_VER_CURRENT);
    CODEGEN_DEFINE(PAPI_NATIVE_MASK);
}

int main(void) {
    codegen();
    return 0;
}
