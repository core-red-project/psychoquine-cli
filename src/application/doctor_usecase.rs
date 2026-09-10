use crate::infrastructure::detector::{RuntimeInfo, SystemDetector};

pub struct DoctorUseCase;

impl DoctorUseCase {
    pub fn execute(&self) -> Vec<RuntimeInfo> {
        SystemDetector::detect_all()
    }
}
