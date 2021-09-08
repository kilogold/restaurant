mod imports;
use imports::renderer::draw;
use std::collections::VecDeque;

type CatalogIndex = usize;
type DialogSelection = usize;

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
        Dialog 
        {
            prompt: String::from("Which option do you want to try?"),
            options: vec!(
                (String::from("This one sounds good."), 1),
                (String::from("Do I have to choose?"), 2),
                (String::from("Let me think about this some more..."), 3),
            ),
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
        
        for (i, optn) in self.interaction.options.iter().enumerate()
        {
            println!("{}) {}", i, optn.0);
        }

        const INVALID_ENTRY : usize = 999;
        let mut dialog_selection: usize = INVALID_ENTRY;
        while dialog_selection == INVALID_ENTRY
        {
            let mut user_input = String::new();
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

                        
            dialog_selection = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's an invalid entry. Try again...");
                    INVALID_ENTRY
                },
            };
        }

        return dialog_selection
    }

    fn tick(&self, sm : &StateMachine)
    {
        let dialog_selection = self.query_input();

        println!("You chose the following option:\n{}", self.interaction.options[dialog_selection].0);

        sm.change_state() //TODO: access catalog from somewhere.
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
        let mut new_catalog = Catalog{
            states : Vec::new(),
        };

        new_catalog.states.push(
            State {
                render: draw::title_screen,
                caption: String::from("Try this!"),
                interaction: Dialog::new(),
                dead_end: false,
            }
        );

        return new_catalog;
    }

    fn first_state(&self) -> &State
    {
        &self.states[0]
    }
}

struct StateMachine<'a>
{
    stack : VecDeque<&'a State>, //Only ever stacks to 2
    is_running : bool
}

impl<'a> StateMachine<'a>
{
    fn change_state(&mut self, new_state : &'a State)
    {
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

    fn tick(&mut self, sm : &StateMachine)
    {
        self.is_running = !self.stack.is_empty();

        if !self.is_running
        {
            return;
        }

        let current_state = self.stack.back().unwrap();

        current_state.tick(sm);

        // If this was a dead end, let's pop that off and return to the previous session.
        if current_state.dead_end
        {
            self.stack.pop_back();
        }
    }

    fn new() -> StateMachine<'a>
    {
        StateMachine 
        {
            stack : VecDeque::new(),
            is_running : true
        }
    }
}

fn main()
{
    let catalog = Catalog::new();
    let mut sm = StateMachine::new();

    sm.change_state(catalog.first_state());

    while sm.is_running 
    {
        sm.tick();
    }    
}


