use std;
use std::f64::consts::PI;

use sound_source::SoundSource;
use vector;

const TABLE_SIZE: usize = 200;

pub struct SineInitArgs {
    frequency: f32,
}

struct SineSource<O> {
    sinetable: [O; TABLE_SIZE],
    left_phase: usize,
    right_phase: usize,
}

impl<O> SoundSource<O> for SineSource<O> where O: std::marker::Copy + std::ops::Div + std::ops::Mul {
    type InitArgs = SineInitArgs;
    fn new(init_args: SineInitArgs) -> SineSource<O> {
        let mut result = SineSource {
            sinetable: [0; TABLE_SIZE],
            left_phase: 0,
            right_phase: 0,
        };

        for i in 0..200 {
            result.sinetable[i] = (i as O / TABLE_SIZE * PI * 2.0).sin();
        }

        result
    }

    fn get_bytes(&mut self, buffer: &mut [O], size: usize) {
        let mut idx: usize = 0;
        for _ in 0..size {
            buffer[idx] = self.sinetable[self.left_phase];
            buffer[idx+1] = self.sinetable[self.right_phase];
            self.left_phase += 1;
            self.left_phase %= TABLE_SIZE;
            self.right_phase +=3;
            self.right_phase %= TABLE_SIZE;
            idx += 2;
        }
    }

    fn get_position(&self) -> vector::Vector3 {
        vector::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}
