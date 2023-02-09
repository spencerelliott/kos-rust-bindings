use crate::pvr;

extern "C" {
    // Debugging functions
    pub fn gdb_init();
    pub fn println(s: &str, ...);

    // PowerVR C functions
    pub fn pvr_init_defaults() -> i32;
    pub fn pvr_init(params: pvr::PvrInitParams) -> i32;
}