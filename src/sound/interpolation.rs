use crate::oscillator::{Clock, Sample};

pub(crate) fn linear(start: Clock, end: Clock, now: Clock) -> Sample {
    ((now - start) / (end - start)) as Sample
}
