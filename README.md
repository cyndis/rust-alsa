# rust-alsa - rustic bindings for libasound ![Build status](https://travis-ci.org/cyndis/rust-alsa.png)

For now, only very basic PCM usage is wrapped.

## Example

This plays one second of a 441Hz tone.

```rust
extern crate alsa;

use alsa::{PCM, Stream, Mode, Format, Access};
use std::f32::consts::PI_2;

fn main() {
    let pcm = PCM::open("default", Stream::Playback, Mode::Blocking).unwrap();
    let mut pcm = pcm.set_parameters(Format::FloatLE, Access::Interleaved, 1, 44100).ok().unwrap();

    let mut buf = [0.0f32; 44100];
    for (idx, sample) in buf.iter_mut().enumerate() {
        let phase = (idx as f32) / 100.0 * PI_2;
        *sample = phase.sin();
    }

    pcm.write_interleaved(&buf).unwrap();
}
```
