use crate::browser::{
    render::RenderTree,
    state::{BrowserLoadingState, BrowserToolbarState},
};

pub(crate) struct BrowserFrame<'a> {
    pub(crate) render_tree: &'a RenderTree,
    pub(crate) toolbar_state: &'a BrowserToolbarState,
    pub(crate) loading_state: BrowserLoadingState,
}

impl<'a> BrowserFrame<'a> {
    pub(crate) fn new(
        render_tree: &'a RenderTree,
        toolbar_state: &'a BrowserToolbarState,
        loading_state: BrowserLoadingState,
    ) -> Self {
        Self {
            render_tree,
            toolbar_state,
            loading_state,
        }
    }
}
