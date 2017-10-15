#![feature(drop_types_in_const)]
#[cfg(test)]
mod tests 
{
    #[test]
    fn it_works() 
    {

         super::Btree::insert(33);             // 1
         super::Btree::insert(23);             // 1
         super::Btree::insert(113);             // 1
         super::Btree::insert(78);             // 1
         super::Btree::insert(7);             // 1

         super::Btree::insert(423);             // 1
         super::Btree::insert(1413);             // 1
         super::Btree::insert(478);             // 1
         super::Btree::insert(74);             // 1
         // super::Btree::node::draw();

         super::Btree::insert(4323);             // 1
         super::Btree::insert(1913);             // 1
         super::Btree::insert(4278);             // 1
         super::Btree::insert(724);             // 1

         super::Btree::insert(3323);             // 1
         super::Btree::insert(9913);             // 1
         super::Btree::insert(2278);             // 1
         super::Btree::insert(2724);             // 1

         super::Btree::node::draw();

        /*
         super::Btree::insert(33);             // 1
         assert!(super::Btree::search(33));

         super::Btree::insert(23);             // 2
         assert!(super::Btree::search(23));

         super::Btree::insert(113);            // 3
         assert!(super::Btree::search(113));

         super::Btree::insert(78);             // 4
         assert!(super::Btree::search(78));

         super::Btree::insert(7);              // 5
         assert!(super::Btree::search(7));

         assert!(super::Btree::search(33));    // 1
         assert!(super::Btree::search(23));    // 2
         assert!(super::Btree::search(113));   // 3
         assert!(super::Btree::search(78));    // 4
         assert!(super::Btree::search(7));     // 5

         super::Btree::node::draw();

         // remove 3
         assert!(super::Btree::search(113));
         super::Btree::delete(113);            // 3

         assert!(super::Btree::search(33));    // 1
         assert!(super::Btree::search(23));    // 2
         assert!(!super::Btree::search(113));  // 3
         assert!(super::Btree::search(78));    // 4
         assert!(super::Btree::search(7));     // 5

         // remove 5
         super::Btree::delete(7);              // 5

         assert!(super::Btree::search(33));    // 1
         assert!(super::Btree::search(23));    // 2
         assert!(!super::Btree::search(113));  // 3
         assert!(super::Btree::search(78));    // 4
         assert!(!super::Btree::search(7));    // 5

         // remove 2
         super::Btree::delete(23);             // 2

         assert!(super::Btree::search(33));    // 1
         assert!(!super::Btree::search(23));   // 2
         assert!(!super::Btree::search(113));  // 3
         assert!(super::Btree::search(78));    // 4
         assert!(!super::Btree::search(7));    // 5

         // remove 1
         super::Btree::delete(33);             // 1

         assert!(!super::Btree::search(33));   // 1
         assert!(!super::Btree::search(23));   // 2
         assert!(!super::Btree::search(113));  // 3
         assert!(super::Btree::search(78));    // 4
         assert!(!super::Btree::search(7));    // 5

         // remove 4
         super::Btree::delete(78);             // 4

         assert!(!super::Btree::search(33));   // 1
         assert!(!super::Btree::search(23));   // 2
         assert!(!super::Btree::search(113));  // 3
         assert!(!super::Btree::search(78));   // 4
         assert!(!super::Btree::search(7));    // 5
         */
    }
}

pub mod  Btree 
{

use std::fmt;
// use std::io;
use std::io::prelude::*;
use std::fs::File;

   pub static mut count: u32 = 0;
   pub static mut  root: Option<Box<node>> = None;

   impl<'a> fmt::Display for node 
   {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
        {
            write!(f, "(id: {}, left: {} right: {})", self.id, printNode(&self.left), printNode(&self.right))
        }
   }

   pub fn printNode(node_: &Option<Box<node>>) -> String
   {
       if node_.is_some()
       {
           return node_.as_ref().unwrap().id.to_string();
       }

       let none = String::from("None");
       none
   }
       
   pub fn search(id: u32) -> bool
   {
       unsafe
       {
           let mut nodeOption = root.as_ref();
           while let Some(node_) = nodeOption
           {
               if node_.id == id
               {
                   return true;
               }
               if id < node_.id
               {
                   nodeOption = node_.left.as_ref();
               }
               else
               {
                   nodeOption = node_.right.as_ref();
               }
           }
       }
       false
   }

    pub fn insert(id: u32)
    {
        unsafe
        {
            println!("insert: {}", id);
            let mut root_ =  root.take();
            root_ = node::insert(root_, id);
            root_.as_mut().unwrap().red = false;
            root = root_.take();
            count = count + 1;
        }
    }

    pub fn delete(id: u32)
    {
        unsafe
        {
            let mut root_ =  root.take();
            root_ = node::delete(root_, id);
            if root_.is_some()
            {
                root_.as_mut().unwrap().red = false;
            }
            root = root_.take();
        }
    }

   pub struct node 
   {
        pub id: u32,
        pub red: bool,
        pub left:  Option<Box< node>>,
        pub right: Option<Box< node>>,
   }

    impl<'a> node
    {
        
        pub fn insert(
                      mut node_: Option<Box<node>>,
                      id: u32 ) -> Option<Box<node>>
         {
            match node_
            {
                None => 
                { Some(Box::new(node{id:id, red: false, left:None, right:None})) },
                Some(x) => 
                {
                    node::insert_(x,id)
                }
            }
        }

        pub fn insert_(
                      // mut self,
                      mut node_: Box<node>,
                      id: u32 ) -> Option<Box<node>>
        {
               if node::isRed(&node_.left) && node::isRed(&node_.right)
               {    
                   node_.colorFlip();
               }


               if id <node_.id 
               {
                   if node_.left.is_none()
                   {
                       node_.left = Some(Box::new(node{id:id, red: false, left:None, right:None}));
                   }
                   else
                   {
                       node_.left = node::insert(node_.left.take(),id );
                   }
               }
               else
               {
                   if node_.right.is_none()
                   {
                       node_.right = Some(Box::new(node{id:id, red: false, left:None, right:None}));
                   } else
                   {
                       node_.right = node::insert(node_.right.take(),id );
                   }
               }

               let node_out: Option<Box<node>>;

               // different from cpp implementation; follows example from slides
               // if !node::isRed(&node_.left)
               if !node::isRed(&node_.left) && node::isRed(&node_.right)
               {
                   println!("rotate left");
                   node_ = node::rotateLeft(node_);
               }

               if node::isRed(&node_.left) && node::isRed(&node_.left.as_ref().unwrap().left)
               {
                   println!("rotate right");
                   node_ = node::rotateRight(node_);
               }

               Some((node_))
        }

        pub fn rotateLeft(
             mut node_: Box<node> ) -> Box<node>
        {


            let mut i = node_.right.take().unwrap();  //1
            i.red = node_.red;   // 4
            node_.red = true;    // 5
            node_.right = i.left.take(); // 2
            i.left = Some(node_);  // 3
            i

            // let mut i = node_.right.take().unwrap();
            // i.red = node_.red;
            // node_.red = true;
            // node_.right = i.left.take();
            // i.left = Some(node_);
            // i
        }

        pub fn rotateRight(
             mut node_: Box<node> ) -> Box<node>
        {
            let mut i = node_.left.take().unwrap();
            i.red = node_.red;
            node_.red = true;
            node_.left = i.right.take();
            i.right= Some(node_);
            i
        }

        fn isRed( node_: &Option<Box<node>>) -> bool
        {
            if node_.is_none()
            {
                false
            }
            else
            {
                node_.as_ref().unwrap().red
            }
        }

        pub fn colorFlip(&mut self) 
        {
            self.red = !self.red;
            
            if self.left.is_some()
            {
                let left = self.left.as_mut().unwrap();
                left.colorFlipNode();
            }

            if self.right.is_some()
            {
                let right = self.right.as_mut().unwrap();
                right.colorFlipNode();
            }
        }

        fn colorFlipNode(&mut self) 
        {
            self.red = !self.red;
        }
        
        pub fn delete(
                      mut node_: Option<Box<node>>,
                      id: u32 ) -> Option<Box<node>>
         {
            match node_
            {
                None => 
                { 
                    return node_
                },
                Some(x) => 
                {
                    let p = node::delete_(x,id);
                    node::fixUp(p)
                }
            }
        }

        pub fn delete_(
                      mut node_: Box<node>,
                      id: u32 ) -> Option<Box<node>>
         {
            if id < node_.id 
            {
                if node_.left.is_some() && !node::isRed(& node_.left) &&
                     node_.right.is_some() && !node::isRed(& node_.left.as_ref().unwrap().left)
                {
                     node_= node::moveRedLeft( node_);
                }
                 node_.left = node::delete( node_.left.take(), id);
            }
            else
            {
                if node::isRed(& node_.left)
                {
                     node_= node::rotateRight(node_);
                }

                if id == node_.id && node_.right.is_none()
                {
                   drop( node_);
                   unsafe
                   {
                       count = count - 1;
                   }
                   return None;
                }

                if node_.right.is_some() && !node::isRed(& node_.right) &&
                     node_.left.is_some() && !node::isRed(& node_.right.as_ref().unwrap().left)
                {
                     node_= node::moveRedRight( node_);
                }
            }

            if id == node_.id
            {
                node_.id =
                {
                    let leftMostNode = node::getMinNode(&node_.right).as_ref().unwrap();
                    leftMostNode.id
                };

                node_.right = node::deleteMinHelper( node_.right.take());
            }
            else
            {
                 node_.right = node::delete( node_.right.take(), id);
            }
            Some(( node_))
         }

        pub fn deleteMinHelper(
            mut node_: Option<Box<node>>) -> Option<Box<node>>
        {
            if node_.as_ref().unwrap().left.is_none()
            {
                drop(node_);
                unsafe
                {
                    count = count - 1;
                }
                return None
            }
            let node_tmp = node_.unwrap();
            if node_tmp.left.is_some() && !node::isRed(&node_tmp.left) &&
                node_tmp.right.is_some() && !node::isRed(&node_tmp.left.as_ref().unwrap().left)
            {
                let node_t = node::moveRedLeft(node_tmp);
                node_ = Some(node_t);
            }
            else
            {
                node_ = Some(node_tmp);
            }
            let mut left = node_.as_mut().unwrap().left.take();
            left = node::deleteMinHelper(left);
            node_.as_mut().unwrap().left = left;
            node::fixUp(node_)
        }

        pub fn fixUp(
            mut node_: Option<Box<node>>) -> Option<Box<node>>
        {
            if node_.is_some()
            {
                if !node::isRed(&node_.as_ref().unwrap().left) && 
                    node::isRed(&node_.as_ref().unwrap().right)
                {
                    node_ = Some(node::rotateLeft(node_.unwrap()));
                }
                if !node::isRed(&node_.as_ref().unwrap().left) && node_.as_ref().unwrap().left.is_some() &&
                    node::isRed(&node_.as_ref().unwrap().left.as_ref().unwrap().left)
                {
                    node_ = Some(node::rotateRight(node_.unwrap()));
                }
            }
            node_
        }

        pub fn getMinNode(
            node_: &Option<Box<node>>) -> &Option<Box<node>>
        {
            if node_.as_ref().unwrap().left.is_none()
            {
                return node_;
            }
            node::getMinNode(node_)
        }

        fn moveRedRight(mut node_: Box<node>) -> Box<node>
        {
            node_.colorFlip();
            if node::isRed(&node_.left.as_ref().unwrap().left) 
            {
                node_= node::rotateRight(node_);
                node_.colorFlip();
            }
            node_
        }

        fn moveRedLeft(mut node_: Box<node>) -> Box<node>
        {
            node_.colorFlip();
            if node::isRed(&node_.right.as_ref().unwrap().left) 
            {
                node_.right = node::rotateRightOption(node_.right.take());
                node_= node::rotateLeft(node_);
                node_.colorFlip();
            }
            node_
        }

        pub fn rotateRightOption(
            mut node_: Option<Box<node>>) -> Option<Box<node>>
        {
            let mut nodeBox = node_.unwrap();
            Some(node::rotateRight(nodeBox))
        }

        pub fn draw()
        {
            let &root_;
            unsafe
            {
                root_ =  root.as_ref().unwrap();
            }

            let mut levels: Vec<Vec<String>> = Vec::new();
            root_.draw_(&mut levels, 1);

            let mut f = match File::create("/home/aa/rust/b-tree/src/foo.txt")
            {
                Err(e) => panic!("Can't open file"),
                Ok(f) => f,
            };

            for outer in &levels
            {
                for inner in outer
                {
                    println!("{}", inner);
                    let result = f.write_all(inner.as_bytes());
                    match result {
                        Err(e) => panic!(e),
                        // Ok(r) => println!("Write successful!"),
                        Ok(r) => (),
                };

                }
            }

            let result = f.flush();
            match result {
                Err(e) => panic!(e),
                Ok(r) => println!("Flush successful!"),
            };
        }

        pub fn draw_(&self, ref mut levels:  &mut Vec<Vec<String>>, level: usize)
        {
            let mut output = String::new();
            if levels.len() < level
            {
                levels.push(Vec::<String>::new());
                if levels.len() != level
                {
                    panic!("nooo00ooowwww!!!! levels.len(): {}, level: {}", levels.len(), level);
                }
            }
            // output = format!("{:^22}\n", format!("node: {}",  self.id));
            // output = format!("{:>7}\n", format!("node: {}",  self.id));
            output = format!("{}\n", format!("//node: {}",  self.id));
            if(self.left.is_some())
            {
                let left = self.left.as_ref().unwrap();
                // output += &format!("{:<11}", format!("left: {}", left.id));
                output += &format!("{}", format!("left: {}\n", left.id));
                left.draw_( levels, (level + 1));
            }
            else
            {
                // output += &format!("{:<11}", format!("left: {}", "None"));
                output += &format!("{}", format!("left: {}\n", "None"));
            }
            if(self.right.is_some())
            {
                let right = self.right.as_ref().unwrap();
                // output += &format!("{:>11}", format!("right: {}\n", right.id));
                output += &format!("{}", format!("right: {}\n", right.id));
                right.draw_( levels, (level + 1));
            }
            else
            {
                // output += &format!("{:<11}", format!("right: {}\n", "None"));
                output += &format!("{}", format!("right: {}\n", "None"));
            }
            levels[level-1].push(output);
        }

     }

 }
