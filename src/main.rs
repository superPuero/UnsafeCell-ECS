use std::cell::UnsafeCell;

mod registry;
mod entity;

fn main(){
    let reg = UnsafeCell::new(registry::Registry::new());
    let registry = registry::Registry::new();
    let ent = entity::Entity::new(&registry);

    println!("{:#?}", ent.transform);

    *ent.transform += 10;

    println!("{}", registry.get::<i32>(ent.id).unwrap());

}