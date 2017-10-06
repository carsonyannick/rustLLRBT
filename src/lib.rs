#![feature(drop_types_in_const)]
#[cfg(test)]
mod tests 
{
    #[test]
    fn it_works() 
    {
         let mut p: Option<Box<super::Btree::node>> = Some(Box::new(super::Btree::node{id:1, red: true, left:None, right:None}));
         // p = p.unwrap().insert_(11);
         // p.unwrap().insert_(11);

         let mut o : Option<Box<super::Btree::node>> = None;
         o = super::Btree::node::insert(o, 44);

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

         println!("First count = {}", unsafe{ super::Btree::count });
/*
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
                   // node_
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
            let mut root_ =  root.take();
            root_ = node::insert(root_, id);
            root_.as_mut().unwrap().red = false;
            root = root_.take();
            println!("insert: root {}", root.as_ref().unwrap());
            count = count + 1;
        }
    }

    pub fn delete(id: u32)
    {
        unsafe
        {
            let mut root_ =  root.take();
            // root_ = node::delete(root_, id);
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
                { Some(Box::new(node{id:id, red: true, left:None, right:None})) },
                Some(x) => 
                {
                    // x.insert_(id)
                    node::insert_(x,id)
                }
            }
        }

        pub fn insert_(
                      // mut self,
                      mut node_: Box<node>,
                      id: u32 ) -> Option<Box<node>>
        {
               if node::isRed(&node_.right) && node::isRed(&node_.left)
               {    
                   node_.colorFlip();
               }


               if id <node_.id 
               {
                   if node_.left.is_none()
                   {
                       node_.left = Some(Box::new(node{id:id, red: true, left:None, right:None}));
                   }
                   else
                   {
                       node_.left = node::insert(node_.left,id );
                   }
               }
               else
               {
                   if node_.right.is_none()
                   {
                       node_.right = Some(Box::new(node{id:id, red: true, left:None, right:None}));
                   }
                   else
                   {
                       //node_.right =node_.right.unwrap().insert(id );
                       node_.right = node::insert(node_.right,id );
                   }
               }

               // different from cpp implementation; follows example from slides
               if !node::isRed(&node_.left)
               {
                   *node_= node_.rotateLeft();
               }

               if node::isRed(&node_.left) && node::isRed(&node_.left.as_ref().unwrap().left)
               {
                   *node_= node_.rotateRight();
               }

               // Some(Box::new(node_))
               Some((node_))
        }

        pub fn rotateLeft(
            mut self ) -> node

        {
            let mut i = self.right.unwrap();
            i.red = self.red;
            self.red = true;
            self.right = i.left.take();
            i.left = Some(Box::new(self));
            *i
        }

        pub fn rotateRight(
            mut self ) -> node
        {
            let mut i = self.left.unwrap();
            i.red = self.red;
            self.red = true;
            self.left = i.right.take();
            i.right = Some(Box::new(self));
            *i
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

        // pub fn new(
        //   id: u32) -> node
        // {
        //     // let leftt: &'a mut Option<&'a mut node<'a>> =  &mut None;
        //     // let rightt: &'a mut Option<&'a mut node<'a>> = &mut None;
        //     // let right: &'a mut Option<&'a mut node<'a>> = &mut None;

        //     node{id:id, red: true, left:None, right:None}
        //     // node{id:id, red: true, left:left, right:right}
        // }


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
        
/*
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
                    let p = x.delete_(id);
                    node::fixUp(p)
                }
            }
            // node::fixUp(node_)
        }

        pub fn delete_(
                      mut self,
                      id: u32 ) -> Option<Box<node>>
         {
            if id < self.id 
            {
                if self.left.is_some() && !node::isRed(&self.left) &&
                    self.right.is_some() && !node::isRed(&self.left.as_ref().unwrap().left)
                {
                    self = node::moveRedLeft(self);
                }
                self.left = node::delete(self.left, id);
            }
            else
            {
                if node::isRed(&self.left)
                {
                    self = self.rotateRight();
                }

                if id == self.id && self.right.is_none()
                {
                   drop(self);
                   unsafe
                   {
                       count = count - 1;
                   }
                   return None;
                }

                if self.right.is_some() && !node::isRed(&self.right) &&
                    self.left.is_some() && !node::isRed(&self.right.as_ref().unwrap().left)
                {
                    self = node::moveRedRight(self);
                }
            }

            if id == self.id
            {

                // let right = &self.right;
                // let leftMostNode = node::getMinNode(&self.right);
                // self.id          = leftMostNode;
                // self.right       = node::deleteMinHelper(self.right);


                {
                    let leftMostNode = node::getMinNode(&self.right).as_ref().unwrap();
                    self.id          = leftMostNode.id;
                }
                self.right       = node::deleteMinHelper(self.right);
            }
            else
            {
                self.right = node::delete(self.right, id);
            }

            Some(Box::new(self))
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
                let node_t = node::moveRedLeft(*node_tmp);
                node_ = Some(Box::new(node_t));
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
*/
        pub fn fixUp(
            mut node_: Option<Box<node>>) -> Option<Box<node>>
        {
            if node_.is_some()
            {
                if !node::isRed(&node_.as_ref().unwrap().left) && 
                    node::isRed(&node_.as_ref().unwrap().right)
                {
                    node_ = Some(Box::new(node_.unwrap().rotateLeft()));
                }

                if !node::isRed(&node_.as_ref().unwrap().left) && node_.as_ref().unwrap().left.is_some() &&
                    node::isRed(&node_.as_ref().unwrap().left.as_ref().unwrap().left)
                {
                    node_ = Some(Box::new(node_.unwrap().rotateRight()));
                }
            }
            node_
        }
/*
        pub fn getMinNode(
            node_: &Option<Box<node>>) -> &Option<Box<node>>
            // node_: &Option<Box<node>>) -> u32
        {
            if node_.as_ref().unwrap().left.is_none()
            {
                return node_;
                // return node_.as_ref().unwrap().id
            }
            node::getMinNode(node_)
        }

        fn moveRedRight(mut self) -> node
        {
            self.colorFlip();
            if node::isRed(&self.left.as_ref().unwrap().left) 
            {
                self = self.rotateRight();
                self.colorFlip();
            }
            self
        }

        fn moveRedLeft(mut self) -> node
        {
            self.colorFlip();
            if node::isRed(&self.right.as_ref().unwrap().left) 
            {
                // self.right.unwrap() = self.right.unwrap().rotateRight();
                self.right = node::rotateRightOption(self.right);
                self = self.rotateLeft();
                self.colorFlip();
            }
            self
        }

        pub fn rotateRightOption(
            mut node_: Option<Box<node>>) -> Option<Box<node>>
        {
            // this whole function needs re-working to use a ref
            //     so that we don't have to reBox after rotateRight()
            let mut nodeBox = node_.unwrap();
            let mut node_t = nodeBox.rotateRight();
            // let mut node_t = nodeBox.rotateRight();
            // *nodeBox = node_t;
            Some(Box::new(node_t))
        }
*/
    }
}
