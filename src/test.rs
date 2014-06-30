use super::*;

#[test]
fn test_sine() {
    let mut pcm = PCM::open("default", Playback, Blocking).unwrap();
    pcm.set_parameters(FloatLE, Interleaved, 1, 44100).ok().unwrap();
    pcm.prepare().ok().unwrap();

    let mut buf = [0.0f32, ..44100];
    for (idx, sample) in buf.as_mut_slice().mut_iter().enumerate() {
        let phase = idx as f32 / 100.0f32 * 2.0 * 3.1415;
        *sample = phase.sin();
    }

    pcm.write_interleaved(buf.as_slice()).ok().unwrap();
}
