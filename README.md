# rust-alsa - rustic bindings for libasound ![Build status](https://travis-ci.org/cyndis/rust-alsa.png)

For now, only very basic PCM usage is wrapped.

## Example

This plays one second of a 441Hz tone.

```rust
extern crate alsa;

use alsa::{PCM, Playback, Blocking, FloatLE, Interleaved};
use std::f32::consts::PI_2;

fn main() {
    let pcm = PCM::open("default", Playback, Blocking).unwrap();
    let mut pcm = pcm.set_parameters(FloatLE, Interleaved, 1, 44100).ok().unwrap();

    let mut buf = [0.0f32, ..44100];
    for (idx, sample) in buf.as_mut_slice().mut_iter().enumerate() {
        let phase = (idx as f32) / 100.0 * PI_2;
        *sample = phase.sin();
    }

    pcm.write_interleaved(buf.as_slice()).unwrap();
}
```
