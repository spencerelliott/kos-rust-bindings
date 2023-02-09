use crate::c_raw;
use crate::common;

pub struct Pvr {}

#[repr(i32)]
pub enum PvrBinSize {
    BINSIZE_0 = 0,
    BINSIZE_8 = 8,
    BINSIZE_16 = 16,
    BINSIZE_32 = 32
}

#[repr(C)]
pub struct PvrBin { }

impl PvrBin {
    pub const OPAQUE: usize = 0;
    pub const OPAQUE_MODIFIER: usize = 1;
    pub const TRANSLUCENT: usize = 2;
    pub const TRANSLUCENT_MODIFIER: usize = 3;
    pub const PUNCHTHRU: usize = 4;
}

#[repr(C)]
pub struct PvrInitParams {
    bin_sizes: [PvrBinSize; 5],
    vertex_buf_size: i32,
    dma_enabled: i32,
    fsaa_enabled: i32
}

impl PvrInitParams {
    pub fn new() -> Self {
        PvrInitParams {
            bin_sizes: [
                PvrBinSize::BINSIZE_16,
                PvrBinSize::BINSIZE_0,
                PvrBinSize::BINSIZE_16,
                PvrBinSize::BINSIZE_0,
                PvrBinSize::BINSIZE_0
            ],
            vertex_buf_size: 512_000,
            dma_enabled: 1,
            fsaa_enabled: 0
        }
    }

    pub fn with_bin_size(mut self, bin: usize, size: PvrBinSize) -> Self {
        self.bin_sizes[bin] = size;
        self
    }

    pub fn with_vertex_size(mut self, kb: i32) -> Self {
        self.vertex_buf_size = kb * 1024;
        self
    }

    pub fn with_dma(mut self, enabled: bool) -> Self {
        self.dma_enabled = if enabled { 1 } else { 0 };
        self
    }
}

impl Pvr {
    /// This will initialize the PowerVR chips with the chosen arguments. The parameters object
    /// will be consumed after being passed into this method.
    pub fn init(params: PvrInitParams) -> common::KosResult {
        unsafe {
            let result = c_raw::pvr_init(params);

            if result == 0 {
                return common::KosResult::Ok;
            } else {
                return common::KosResult::Error("Could not initialize PVR with the specified parameters");
            }
        }
    }

    pub fn init_defaults() -> common::KosResult {
        unsafe {
            let result = c_raw::pvr_init_defaults();

            if result == 0 {
                return common::KosResult::Ok;
            } else {
                return common::KosResult::Error("Could not initialize PVR with defaults");
            }
        }
    }
}