/*
 * Copyright 2018 German Research Center for Artificial Intelligence (DFKI)
 * Author: Clemens Lutz <clemens.lutz@dfki.de>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn do_papi_init() {
        let ver = unsafe {
            PAPI_library_init(_papi_ver_current)
        };
        assert_eq!(ver, _papi_ver_current);

        let is_inited = unsafe {
            PAPI_is_initialized()
        };
        assert_ne!(is_inited, PAPI_NOT_INITED as i32);
    }

    #[test]
    fn get_real_cyc() {
        let cycles = unsafe {
            PAPI_get_real_cyc()
        };
        assert!(cycles >= 0);
    }

    #[test]
    fn get_num_counters() {
        let num_hwcntrs = unsafe {
            PAPI_num_counters()
        };
        assert!(num_hwcntrs >= 0);
    }
}
