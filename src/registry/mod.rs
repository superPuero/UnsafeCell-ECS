use std::{any::{type_name, Any, TypeId}, cell::UnsafeCell, collections::HashMap};

type EntityID = u64;
type ComponentID = TypeId;

pub struct RegistryData{
    counter: u64,
    valid: HashMap<EntityID, Vec<ComponentID>>,   
    data: HashMap<(u64, TypeId), Box<dyn Any>>,
}

impl RegistryData{
    fn new() -> RegistryData{
        RegistryData{
            counter: 0,
            valid: HashMap::new(),
            data: HashMap::new(),
        }
    }
}


pub struct Registry{
    data: UnsafeCell<RegistryData>
}

impl Registry{
    pub fn new() -> Self{
        Self{
            data: UnsafeCell::new(RegistryData::new())
        }
    }

    pub fn create(&self) -> EntityID{
        let slf = unsafe { &mut *self.data.get() }; 
        slf.counter += 1;
        slf.valid.insert(slf.counter, Vec::new());
        slf.counter
    }

    pub fn add<T: 'static>(&self, entity: EntityID, component: T){
        let slf = unsafe { &mut *self.data.get() }; 
        let type_id = TypeId::of::<T>();
        slf.valid.get_mut(&entity).unwrap().push(type_id);
        slf.data.insert((entity, type_id), Box::new(component));
    }

    pub fn get<T: 'static>(&self, entity: EntityID) -> Option<&T>{
        let type_id = TypeId::of::<T>();
        match unsafe { &*self.data.get() }.data.get(&(entity, type_id)){
            Some(component) =>{
                let component = unsafe { &*(component.as_ref() as *const dyn Any as *const T) };
                Some(component)
            }
            None => None,
        }
    }

    pub fn get_uncheked<T: 'static>(&self, entity: EntityID) -> &T{
        let type_id = TypeId::of::<T>();
        match unsafe { &*self.data.get() }.data.get(&(entity, type_id)){
            Some(component) =>{
                unsafe { &*(component.as_ref() as *const dyn Any as *const T) }
            }
            None => panic!("get_uncheked failed with entity: {} and component: {}", entity, type_name::<T>()),
        }
    }

    pub fn get_mut<T: 'static>(&self, entity: EntityID) -> Option<&mut T>{
        let type_id = TypeId::of::<T>();
        match unsafe { &mut *self.data.get() }.data.get_mut(&(entity, type_id)){
            Some(component) =>{
                let component = unsafe { &mut *(component.as_mut() as *mut dyn Any as *mut T) };
                Some(component)
            }
            None => None,
        }
    }

    
    pub fn get_mut_uncheked<T: 'static>(&self, entity: EntityID) -> &mut T{
        let type_id = TypeId::of::<T>();
        match unsafe { &mut *self.data.get() }.data.get_mut(&(entity, type_id)){
            Some(component) =>{
                unsafe { &mut *(component.as_mut() as *mut dyn Any as *mut T) }
            }
            None => panic!("get_mut_uncheked failed with entity: {} and component: {}", entity, type_name::<T>()),
        }
    }
}