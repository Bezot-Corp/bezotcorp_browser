#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavigationCommand {
    Back,
    Forward,
    Reload,
}
