pub trait BrowserEngine {
    fn name(&self) -> &'static str;
    fn current_url(&self) -> &str;
    fn load_url(&mut self, url: &str);
    fn reload(&mut self);
    fn go_back(&mut self);
    fn go_forward(&mut self);
}
