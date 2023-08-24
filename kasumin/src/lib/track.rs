
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TrackMetadata<'track> {
    pub path: &'track str
}