// Copyright 2018-2019 German Research Center for Artificial Intelligence (DFKI)
// Copyright 2019 Yeonsoo Kim
//
// Authors:
//   Clemens Lutz <clemens.lutz@dfki.de>
//   Yeonsoo Kim <alkorang@outlook.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[cfg(test)]
mod tests {

    use super::*;

    fn do_papi_init() {
        unsafe {
            let ver = PAPI_library_init(PAPI_VER_CURRENT);
            assert_eq!(ver, PAPI_VER_CURRENT);
        }

        let is_inited = unsafe { PAPI_is_initialized() };
        assert_ne!(is_inited, PAPI_NOT_INITED as i32);
    }

    fn get_real_cyc() {
        let cycles = unsafe { PAPI_get_real_cyc() };
        assert!(cycles >= 0);
    }

    fn get_num_counters() {
        let num_hwcntrs = unsafe { PAPI_num_counters() };
        assert!(num_hwcntrs >= 0);
    }

    #[test]
    fn do_simple_test() {
        do_papi_init();
        get_real_cyc();
        get_num_counters();
    }
}
