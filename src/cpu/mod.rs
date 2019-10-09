#[derive(Clone)]
pub struct Flags {
    pub negative: bool,
    pub zero: bool,
    pub carry: bool,
    pub signed_overflow: bool
}