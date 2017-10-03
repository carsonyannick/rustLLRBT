#![feature(drop_types_in_const)]
#[cfg(test)]
mod tests 
{
    #[test]
    fn it_works() 
    {
         let mut p: Option<Box<super::Btree::node>> = Some(Box::new(super::Btree::node{id:1, red: true, left:None, right:None}));
         p = p.unwrap().insert_(11);
         p.unwrap().insert_(11);

         let mut o : Option<Box<super::Btree::node>> = None;
         o = super::Btree::node::insert(o, 44);

         super::Btree::insert(33);
         assert!(super::Btree::search(33));
         super::Btree::insert(34);
         assert!(super::Btree::search(34));
         super::Btree::insert(35);
         assert!(super::Btree::search(35));
         super::Btree::insert(36);
         assert!(super::Btree::search(36));
         super::Btree::insert(37);
         assert!(super::Btree::search(37));
    }
}

pub mod  Btree 
{

   static mut count: u32 = 0;
   static mut  root: Option<Box<node>> = None;

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
        }
    }

   pub struct node 
   {
        pub id: u32,
        pub red: bool,
        pub left:  Option<Box< node>>,
        pub right: Option<Box< node>>,
   }

    impl node
    {
        
        pub fn insert(
                      mut node_: Option<Box<node>>,
                      id: u32 ) -> Option<Box<node>>
         {
            match node_
            {
                None => 
                { 
                    unsafe 
                    {
                        count = count + 1;
                    }
                    Some(Box::new(node{id:id, red: true, left:None, right:None}))
                },
                Some(x) => 
                {
                    x.insert_(id)
                }
            }
        }

        pub fn insert_(
                      mut self,
                      id: u32 ) -> Option<Box<node>>
        {
               if node::isRed(&self.right) && node::isRed(&self.left)
               {    
                   self.colorFlip();
               }


               if id < self.id 
               {
                   if self.left.is_none()
                   {
                       self.left = Some(Box::new(node{id:id, red: true, left:None, right:None}));
                   }
                   else
                   {
                       self.left = node::insert(self.left,id );
                   }
               }
               else
               {
                   if self.right.is_none()
                   {
                       self.right = Some(Box::new(node{id:id, red: true, left:None, right:None}));
                   }
                   else
                   {
                       // self.right = self.right.unwrap().insert(id );
                       self.right = node::insert(self.right,id );
                   }
               }

               // different from cpp implementation; follows example from slides
               if !node::isRed(&self.left)
               {
                   self = self.rotateLeft();
               }

               if node::isRed(&self.left) && node::isRed(&self.left.as_ref().unwrap().left)
               {
                   self = self.rotateRight();
               }

               Some(Box::new(self))
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

        pub fn new(
          id: u32) -> node
        {
            // let leftt: &'a mut Option<&'a mut node<'a>> =  &mut None;
            // let rightt: &'a mut Option<&'a mut node<'a>> = &mut None;
            // let right: &'a mut Option<&'a mut node<'a>> = &mut None;

            node{id:id, red: true, left:None, right:None}
            // node{id:id, red: true, left:left, right:right}
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
                // self.left = node::delete_(self.left, id);
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
                let leftMostNode = node::getMinNode(&self.right).as_ref().unwrap();
                self.id          = leftMostNode.id;
                // self.right       = node::delete(self.right);
            }
            // else
            // {

            None
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

    }
}
