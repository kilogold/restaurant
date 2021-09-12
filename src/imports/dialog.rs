pub type CatalogIndex = usize;
pub type DialogSelection = usize;

pub struct Dialog
{
    pub prompt: String,

    // String: option description.
    // CatalogIndex: index into immutable State catalog array. (the sate machine hosts a stack)
    pub options: Vec<(String,CatalogIndex)>,
}

impl Dialog
{
    pub fn new() -> Dialog
    {
        Dialog{
            options : Vec::new(),
            prompt : String::new(),
        }
    }
}
