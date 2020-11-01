#[derive(Clone, Copy, Default)]
pub struct Context {
    pub scale: f32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            scale: 2.,
        }
    }
}