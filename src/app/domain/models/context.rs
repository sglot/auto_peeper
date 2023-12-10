#[derive(Default, Debug, Clone)]
pub struct Context {
    pub next_round_drom: bool,
    pub next_round_avito: bool,

}

impl Context {
    pub fn new() -> Self { Self { next_round_drom: false, next_round_avito: false} }

    
}



