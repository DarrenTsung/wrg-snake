use wrg_2d::IntVector2;

pub struct Food {
    pos: IntVector2,
}

impl Food {
    pub fn new(pos: IntVector2) -> Food {
        Food { pos }
    }

    pub fn pos(&self) -> &IntVector2 {
        &self.pos
    }
}
