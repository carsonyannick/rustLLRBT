extern crate btree;

fn main()
{
    let serverOption = btree::socket::server::new(String::from("/tmp/rustLLRBTSocket"));
    let mut client = serverOption.accept();
    
    loop
    {
        let input = client.listen();
        // println!("command {:?} argument {:?}", input.command, input.argument);

        if input.command_is(b"add")
        {
            println!("inside add()");
            // btree::Btree::insert(input.argument,input.command);
        }
        else if input.command_is(b"search")
        {
            println!("inside search()");
            btree::Btree::search(input.argument);
        }
        else if input.command_is(b"delete")
        {
            println!("inside delete()");
            btree::Btree::delete(input.argument);
        }
        else if input.command_is(b"draw")
        {
            println!("inside draw()");
            client.send(btree::Btree::node::draw());
        }
    }
}
