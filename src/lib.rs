// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
mod front_of_house 
{
    pub mod hosting 
    {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() 
{
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


pub fn serve_order() 
{
    println!("Cooking order");
}

pub mod back_of_house 
{
    pub fn fix_incorrect_order() 
    {
        cook_order();
        super::serve_order();
    }

    fn cook_order() 
    {
    }
}
