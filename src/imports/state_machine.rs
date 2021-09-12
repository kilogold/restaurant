use super::dialog::CatalogIndex;
use super::state::State;
use super::catalog::Catalog;
use std::collections::VecDeque;

pub struct StateMachine<'a>
{
    pub catalog : &'a Catalog,
    pub stack : VecDeque<&'a State>, //Only ever stacks to 2
    pub is_running : bool
}

impl StateMachine<'_>
{
    pub fn get_state(& self, state_index : CatalogIndex) -> & State
    {
        &self.catalog.states[state_index]
    }

    pub fn change_state(& mut self, state_index : CatalogIndex)
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

    pub fn tick(&mut self)
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

    pub fn new(catalog_in : &Catalog) -> StateMachine
    {
        StateMachine 
        {
            catalog: catalog_in,
            stack : VecDeque::new(),
            is_running : true
        }
    }
}