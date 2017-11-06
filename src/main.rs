extern crate btree;

fn main()
{
    let serverOption = btree::socket::server::new(String::from("/tmp/rustLLRBTSocket"));

    loop
    {
        let mut client = serverOption.accept();
        
        #[cfg(feature="debug")]
        {
            println!("debugging...");
        }

        let input = client.listen();
        // println!("command {:?} argument {:?}", input.command, input.argument);

        if input.command_is(b"add")
        {
            println!("inside add(), id {}, data {}",input.id,::std::str::from_utf8(&input.data).unwrap());

            btree::Btree::insert(input.id,&input.data);
            client.send(String::from("Added!"));
        }
        else if input.command_is(b"search")
        {
            println!("inside search(), id {}, data {}",input.id,::std::str::from_utf8(&input.data).unwrap());
            let result = btree::Btree::search(input.id);

            let mut reply = String::new();
            
            match result
            {
                None => 
                {
                    reply.push_str("not found");
                },
                Some(x) => 
                {
                    // println!("search result: {:?}", &x);
                    reply = format!("found {}", ::std::str::from_utf8(&x).unwrap());
                }
            }

            println!("reply: {}", reply);
            client.send(reply);
        }
        else if input.command_is(b"delete")
        {
            println!("inside delete() id: {}", input.id);
            btree::Btree::delete(input.id);
        }
        else if input.command_is(b"print")
        {
            println!("inside draw()");
            client.send(btree::Btree::node::printInOrder());
        }
        else if input.command_is(b"draw")
        {
            println!("inside draw()");
            let reply = btree::Btree::node::printInOrder();
            println!("draw:\n{}", reply);
            client.send(reply);
        }
    }
}
