use super::registry;

pub struct Entity<'a>{
    pub id: u64,
    pub transform: &'a mut i32
}
impl<'a> Entity<'a> {

    pub fn new(ctx: &'a registry::Registry) -> Entity<'a>{
        let new_id = ctx.create();
        ctx.add(new_id, 243i32);

        Entity{
            id: new_id,
            transform:  {
                let l = ctx.get_mut::<i32>(new_id).unwrap();
                println!("GOT {:#?}", l);
                l
            }
        }

    }
}