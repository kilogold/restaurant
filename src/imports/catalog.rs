
use crate::imports::state::State;
use crate::imports::renderer::draw;
use crate::imports::dialog::{Dialog};

pub struct Catalog
{
    pub states : Vec<State>, //All of the possible states,
}

impl Catalog
{
    pub fn new() -> Catalog
    {
        Catalog{
            states : vec!(
                State {
                    render: draw::title_screen,
                    caption: String::from("Try this!"),
                    interaction: Dialog {
                        prompt: String::from("Which option do you want to try?"),
                        options: vec!(
                            (String::from("This one sounds good."), 1),
                            (String::from("Do I have to choose?"), 2),
                            (String::from("Let me think about this some more..."), 3),
                        ),
                    },
                    dead_end: false,
                },
                State {
                    render: draw::blank,
                    caption: String::from("That one sounds great! I think we should go back..."),
                    interaction: Dialog::new(),
                    dead_end: true,
                }
            )
        }
    }

    fn first_state(&self) -> &State
    {
        &self.states[0]
    }
}