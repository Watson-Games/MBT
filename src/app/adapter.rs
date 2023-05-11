use crate::app::result::MBTResult;
use crate::GenParams;

pub struct MBT;

impl MBT {
    pub fn new() -> Self {
        Self {}
    }

    pub fn gen<'a>(&self, p: &GenParams<'a>) -> MBTResult<()> {
        todo!()
    }
}

impl Default for MBT {
    fn default() -> Self {
        Self::new()
    }
}
