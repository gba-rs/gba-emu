use crate::memory::work_ram::WorkRam;

pub struct GPU {
    pub not_used_mem_2: WorkRam,
}

impl GPU {
    pub fn new() -> GPU {
        return GPU {
            not_used_mem_2: WorkRam::new(0xFFFFBF, 0)
        };
    }
}