use crate::StateMachine;
use super::dialog::{Dialog, DialogSelection};
use super::renderer::draw;

use std::io;
use std::io::prelude::{Read, Write};

const INVALID_SELECTION : usize = 999;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub struct State
{
   pub render: fn(),
   pub caption: String,
   pub interaction: Dialog,
   pub dead_end: bool,
}

impl State
{
    fn new() -> State 
    {
        State 
        {
            render: draw::blank,
            caption: String::new(),
            interaction: Dialog::new(),
            dead_end: false
        }
    }

    pub fn enter(&self)
    {
        draw::clear_screen();
        (self.render)();
        println!("{}",self.caption);
    }

    pub fn query_input(&self) -> DialogSelection
    {
        println!("{}",self.interaction.prompt);
        
        if self.interaction.options.len() == 0
        {
            pause();
            return INVALID_SELECTION;
        }

        for (i, optn) in self.interaction.options.iter().enumerate()
        {
            println!("{}) {}", i, optn.0);
        }

        let mut dialog_selection: usize = INVALID_SELECTION;
        while dialog_selection == INVALID_SELECTION
        {
            let mut user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

                        
            dialog_selection = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's an invalid entry. Try again...");
                    INVALID_SELECTION
                },
            };
        }

        return dialog_selection
    }

    pub fn tick(&self, sm : &mut StateMachine<'_>)
    {
        let dialog_selection = self.query_input();

        if dialog_selection != INVALID_SELECTION
        {
            let selection_tuple = &self.interaction.options[dialog_selection];
            println!("You chose the following option:\n{}", selection_tuple.0);

            sm.change_state(selection_tuple.1)
        }
    }
}