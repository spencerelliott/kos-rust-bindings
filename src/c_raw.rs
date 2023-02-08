use crate::pvr;

extern "C" {
    pub fn gdb_init();
    pub fn pvr_init_defaults();
    pub fn println(s: &str, ...);
}