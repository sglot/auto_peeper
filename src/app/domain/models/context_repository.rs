use crate::CONTEXT;


#[derive(Default)]
pub struct ContextRepository {}

impl ContextRepository {
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_true() {
        let mut lock = CONTEXT.lock().unwrap();
        lock.next_round_drom = true;
        lock.next_round_avito = true;
    }

    pub fn set_default() {
        let mut lock = CONTEXT.lock().unwrap();
        lock.next_round_drom = false;
        lock.next_round_avito = false;
    }

    pub fn next_round() -> bool {
        let lock = CONTEXT.lock().unwrap();
        return lock.next_round_drom && lock.next_round_avito;
    }

    pub fn set_next_round_drom(value: bool) {
        CONTEXT.lock().unwrap().next_round_drom = value;
    }

    pub fn set_next_round_avito(value: bool) {
        CONTEXT.lock().unwrap().next_round_avito = value;
    }

}
