
#[derive(PartialEq, Eq)]
pub enum OptionState {
    BinaryToDecimal,
    DecimalToBinary,
}

pub struct SelectedOption {
    pub state: OptionState,
    pub locked: bool,
}

impl SelectedOption {
    pub fn new() -> SelectedOption {
        SelectedOption {
            state: OptionState::BinaryToDecimal,
            locked: false,
        }
    }

    pub fn toggle(&mut self) {
        if self.locked { return }
        match self.state {
            OptionState::BinaryToDecimal => {
                self.state = OptionState::DecimalToBinary;
            },
            OptionState::DecimalToBinary => {
                self.state = OptionState::BinaryToDecimal;
            }
        }
    }
}

pub struct SelectedNumber {
    pub number: String,
}

impl SelectedNumber {
    pub fn new() -> SelectedNumber {
        SelectedNumber {
            number: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        // handle adding before open
        self.number.push(c);
    }

    pub fn pop(&mut self) {
        // hangle removeing before open
        self.number.pop();
    }

}
    
pub struct State {
    pub selected_number: SelectedNumber,
    pub selected_option: SelectedOption,
    pub should_render: bool,
    pub should_quit: bool,
}

impl State {
    pub fn new() -> State {
        State {
            selected_number: SelectedNumber::new(),
            selected_option: SelectedOption::new(),
            should_render: false,
            should_quit: false,
        }
    }
}