pub struct State {

    exit: bool
}

impl State {
    pub fn new() -> State {
        State { 
            exit: false
        }
    }
    pub fn exit(&mut self) { self.exit = true }
    pub fn exiting(&self) -> bool { self.exit == true }
}