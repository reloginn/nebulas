#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Event {
    Destroy { to: &'static [u8] },
}
