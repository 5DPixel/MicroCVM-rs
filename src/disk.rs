const DISK_SIZE: usize = 1024 * 1024 * 8;

pub struct MicroCVMDisk {
    pub data: Vec<u8>,
    pub filepath: String,
}

impl MicroCVMDisk {
    pub fn empty() -> Self {
        Self {
            data: vec![0; DISK_SIZE],
            filepath: String::from(""),
        }
    }
}
