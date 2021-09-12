mod imports;
use imports::state_machine::StateMachine;
use imports::catalog::Catalog;

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


