mod imports;
use imports::renderer::draw;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

type CatalogIndex = usize;
type DialogSelection = usize;

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

struct Dialog
{
    prompt: String,

    // String: option description.
    // CatalogIndex: index into immutable State catalog array. (the sate machine hosts a stack)
    options: Vec<(String,CatalogIndex)>,
}

impl Dialog
{
    fn new() -> Dialog
    {
        Dialog{
            options : Vec::new(),
            prompt : String::new(),
        }
    }
}

struct State
{
    render: fn(),
    caption: String,
    interaction: Dialog,
    dead_end: bool,
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

    fn query_input(&self) -> DialogSelection
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

    fn tick(&self, sm : &mut StateMachine<'_>)
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

struct Catalog
{
    states : Vec<State>, //All of the possible states,
}

impl Catalog
{
    fn new() -> Catalog
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

struct StateMachine<'a>
{
    catalog : &'a Catalog,
    stack : VecDeque<&'a State>, //Only ever stacks to 2
    is_running : bool
}

impl StateMachine<'_>
{
    fn get_state(& self, state_index : CatalogIndex) -> & State
    {
        &self.catalog.states[state_index]
    }
    fn change_state(& mut self, state_index : CatalogIndex)
    {
        //let new_state = StateMachine::get_state(self, state_index); //[E0495]
        //let new_state = self.get_state(state_index); //[E0495]
        let new_state = &self.catalog.states[state_index]; //idk why I can't use get_state....

        if new_state.dead_end || self.stack.is_empty()
        {
            self.stack.push_back(new_state);
        }
        else
        {
            self.stack[0] = new_state;
        }

        new_state.enter();
    }

    fn tick(&mut self)
    {
        self.is_running = !self.stack.is_empty();

        if !self.is_running
        {
            return;
        }

        // If this was a dead end, let's pop that off and return to the previous session.
        let current_state = self.stack.back_mut().unwrap();
        let dead_end_precheck = current_state.dead_end;

        current_state.tick(self);

        if dead_end_precheck
        {
            self.stack.pop_back();
            self.stack.back_mut().unwrap().enter();
        }
    }

    fn new(catalog_in : &Catalog) -> StateMachine
    {
        StateMachine 
        {
            catalog: catalog_in,
            stack : VecDeque::new(),
            is_running : true
        }
    }
}

fn main()
{
    let catalog = Catalog::new();
    let mut sm = StateMachine::new(&catalog);

    sm.change_state(0);

    while sm.is_running 
    {
        sm.tick();
    }    
}


