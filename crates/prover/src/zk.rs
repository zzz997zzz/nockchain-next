
pub trait ZkBackend {
    fn setup(&self);
    fn prove(&self, input: &[u8]) -> Vec<u8>;
    fn verify(&self, proof: &[u8]) -> bool;
}
