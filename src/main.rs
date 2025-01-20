mod registry;
mod entity;

fn main(){
    let registry = registry::Registry::new();
    let ent = entity::Entity::new(&registry);

    println!("{:#?}", ent.transform);

    *ent.transform += 10;

    println!("{}", registry.get::<i32>(ent.id).unwrap());
}