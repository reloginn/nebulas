#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Event {
    Shutdown { to: usize },
    Freeze { to: usize, on: std::time::Duration },
}
