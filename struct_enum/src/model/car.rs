

struct Engine {
    id:u32,
    name:String,
    core:Core,



}

impl Engine {

    // &self replace engine:&Engine
    // &self is a immutable borrow
    pub fn to_string (&self)->String{
        let engine_str = String::from("this is a very cool engine");
        engine_str
    }

    //
    pub fn set_engine_name(&mut self,name:String){
        self.name = String.from(name);
    }
}

struct Core {
    worker : u8,
}