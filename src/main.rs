mod imports;
use imports::renderer::draw;
use std::collections::VecDeque;

struct dialog
{
    prompt: String,

    // String: option description.
    // u32: index into immutable state catalog array. (the sate machine hosts a stack)
    options: Vec<(String,u32)>,
}

impl dialog
{
    fn new() -> dialog 
    {
        dialog 
        {
            prompt: String::from("Try this!"),
            options: Vec::new(),
        }
    } 
}

struct state
{
    render: fn(),
    caption: String,
    interaction: dialog,
    dead_end: bool,
}

impl state
{
    fn new() -> state 
    {
        state 
        {
            render: draw::blank,
            caption: String::new(),
            interaction: dialog::new(),
            dead_end: false
        }
    }

    pub fn enter(&mut self)
    {
        (self.render)();
        println!("{}",self.caption);
    }

    fn query_input(&mut self)
    {

    }

    fn tick(&self)
    {

    }
}

struct state_machine<'a>
{
    catalog : Vec<state>, //All of the possible states
    stack : VecDeque<&'a state>, //Only ever stacks to 2
    is_running : bool
}

impl<'a> state_machine<'a>
{
    // fn change_state(&mut self, new_state : state)
    // {
    //     if(new_state.dead_end)
    //     {
    //         self.stack.push_back();
    //     }
    // }

    fn tick(&mut self)
    {
        self.is_running = !self.stack.is_empty();

        if !self.is_running
        {
            return;
        }

        let current_state : &state = self.stack.back().expect("stack should never be empty.");

        current_state.tick();
    }

    fn init_catalog(catalg : &'a mut Vec<state>)
    {
        let obj = state {
            render: draw::title_screen,
            caption: String::from("Try this!"),
            interaction: dialog::new(),
            dead_end: false,
        };

        catalg.push(obj);
    }

    fn init_stack(vd : &mut VecDeque<&'a state>,  catalg : &'a Vec<state>)
    {
        vd.push_back(& catalg[0]);
    }

    fn new() -> state_machine<'a>
    {
        state_machine 
        {
            catalog : Vec::new(),
            stack : VecDeque::new(),
            is_running : true
        }
    }
}

fn main()
{
    // draw::clear_screen();
    // draw::title_screen();
    // println!("
    // Welcome to the restaurant!
    // How may we serve you today?");


    // let obj = state {
    //     render: draw::title_screen,
    //     caption: String::from("Try this!"),
    //     interaction: dialog::new(),
    //     dead_end: false,
    // };

    // (obj.render)();

    // let mut sm = state_machine::new();

    // {
    //     let ctlg = &mut sm.catalog;
    //     state_machine::init_catalog(ctlg);
    //     state_machine::init_stack(&mut sm.stack, ctlg);
    // }

    // while sm.is_running 
    // {
    //     sm.tick();
    // }    
    
    let mut s = state_machine::new();
    {
        let r1 = &mut s.catalog;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    r2.tick();


}


