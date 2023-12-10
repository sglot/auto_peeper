#[derive(Clone, Copy)]
pub enum PeeperClientType {
    Drom,
    Avito,
}

pub struct PeeperClientTypeData {
    pub enum_name: PeeperClientType,
    pub name: String,
}

impl PeeperClientTypeData {
    pub fn new(enum_name: PeeperClientType) -> Self { Self { enum_name,  name: PeeperClientTypeData::to_string(&enum_name)} }

    pub fn to_string(t: &PeeperClientType) -> String {
        match t {
           PeeperClientType::Drom => "drom".to_string(),
           PeeperClientType::Avito => "avito".to_string(),
        }
    }
}
