
use crate::zk::ZkBackend;

pub struct DefaultBackend;

impl ZkBackend for DefaultBackend {
    fn setup(&self) { println!("Default setup"); }
    fn prove(&self, _: &[u8]) -> Vec<u8> { vec![0] }
    fn verify(&self, _: &[u8]) -> bool { true }
}
