#[cfg(test)]

pub mod socket;

mod tests 
{
use super::Btree;
use super::socket;

    #[test]
    fn it_works() 
    {

    }
}

pub mod Btree 
{
   pub struct node 
   {
        pub id: u32,
        pub red: bool,
        pub left:  Option<Box< node>>,
        pub right: Option<Box< node>>,
   }

}
