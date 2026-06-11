#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum BrowserLoadingState {
    #[default]
    Idle,
    Loading,
}
