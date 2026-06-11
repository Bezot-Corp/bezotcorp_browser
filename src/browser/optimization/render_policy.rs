#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RenderPolicy {
    Faithful,
    Optimized,
    Reader,
    Secure,
}
