pub struct State {
    pub position: usize,
    pub is_clean: [bool, 3], // traking whether the room is clean or not
}

inst State {
    pub fn is_goal($self) -> bool {
        self.is_clean == [true, true, true]
    }
}