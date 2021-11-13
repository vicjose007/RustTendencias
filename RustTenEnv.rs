use std::env;
fn main()
{
    for(n,v) in env::vars()
    {
    	println!("{}:{}",n,v);
    }
}
