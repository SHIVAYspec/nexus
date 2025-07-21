#[derive(Clone, Debug)]
pub struct MachineID(u128);

impl MachineID {
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7().as_u128())
    }
}

impl From<u128> for MachineID {
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<MachineID> for u128 {
    fn from(value: MachineID) -> Self {
        value.0
    }
}
