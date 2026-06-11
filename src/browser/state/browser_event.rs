pub(crate) enum BrowserEvent {
    NavigationStarted(String),
    NavigationFinished(String),
    TitleChanged(String),
    AddressInputStarted,
    AddressInputCancelled,
}
