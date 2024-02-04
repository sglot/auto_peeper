#[derive(Default, Debug, Clone)]
pub struct Context {
    pub next_round_drom: bool,
    pub next_round_avito: bool,
    pub enable_avito: bool,
    pub enable_drom: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            next_round_drom: false,
            next_round_avito: false,
            enable_avito: true,
            enable_drom: true,
        }
    }
}
