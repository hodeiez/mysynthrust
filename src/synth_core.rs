use nih_plug::prelude::*;

use rand_pcg::Pcg32;
use std::sync::Arc;

use crate::{params::MySynthParams, voice::Voice, NUM_VOICES};

pub struct MySynth {
    pub params: Arc<MySynthParams>,

    /// A pseudo-random number generator. This will always be reseeded with the same seed when the
    /// synth is reset. That way the output is deterministic when rendering multiple times.
    pub prng: Pcg32,
    /// The synth's voices. Inactive voices will be set to `None` values.
    pub voices: [Option<Voice>; NUM_VOICES as usize],
    /// The next internal voice ID, used only to figure out the oldest voice for voice stealing.
    /// This is incremented by one each time a voice is created.
    pub next_internal_voice_id: u64,
}
