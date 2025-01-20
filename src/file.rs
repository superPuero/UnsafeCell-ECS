pub mod foo{
    pub struct  Foo{
        x: u32,
    }
    impl Foo{
        pub fn new(num: u32) -> Self{
            return Self{x: num};
        }
        pub fn print(&self){
            println!("{}", self.x);
        }
    }   
}