#[derive(Default, Clone, Copy, Debug)]
pub struct Clock(pub u64);

impl Clock {
  pub fn tick(self, elapsed: u64) -> Self {
    if self.0.saturating_add(elapsed) == u64::MAX {
      Self(0)
    }
    else {
      Self(self.0 + elapsed)
    }
  }
}
