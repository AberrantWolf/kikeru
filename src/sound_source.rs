use vector;

// SoundSource wraps any sound format source with meta information and functionality such as
// position, speed, direction, and methods for acquiring the next set of samples.
pub trait SoundSource<O> {
    type InitArgs;
    fn new(init_args: Self::InitArgs) -> Self;
    fn get_bytes(&mut self, buffer: &mut [O], size: usize);
    fn get_position(&self) -> vector::Vector3;
}
