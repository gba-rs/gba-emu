struct Range {
    pub lower: u32,
    pub upper: u32
}

impl Range {
    pub fn new(lower: u32, upper: u32) -> Range {
        return Range {
            lower: lower,
            upper: upper
        };
    }
}