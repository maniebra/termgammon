use super::Player;

pub struct Cube {
    pub value: u32,
    pub owner: Option<Player>,
}

impl Default for Cube {
    fn default() -> Self {
        Self { value: 1, owner: None }
    }
}

impl Cube {
    pub fn can_double(&self, p: Player) -> bool {
        self.owner != Some(p.other())
    }

    pub fn take(&mut self, p: Player) {
        self.value *= 2;
        self.owner = Some(p);
    }
}
