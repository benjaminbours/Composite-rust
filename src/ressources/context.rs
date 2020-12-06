#[derive(Clone, Copy, Default)]
pub struct Context {
    pub width: f32,
    pub height: f32,
}

impl Context {
    pub fn new() -> Context {
        Context {
            width: 100.,
            height: 100.,
        }
    }
}