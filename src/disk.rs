const DISK_SIZE: usize = 32 * 1024;

pub struct MicroCVMDisk {
    pub data: Vec2<u8>,
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
