use std::rc::Rc;

#[derive(Clone, Hash, Debug)]
pub struct ObjString {
    string_rc : Rc<String>,
}

impl ObjString {
    pub fn from_str(s: &str) -> Self {
        Self { string_rc: Rc::new(s.to_string())}
        //Self{Rc::new(s.to_string())}
    }
}

/*
    Object to support custom data types in the VM
    Currently, it only supports strings
    More types can be added in the future
*/
#[derive(Clone, Debug)]
pub enum Object {
    ObjString(ObjString),
}

impl Object{
    // pub fn from_str(s: &str) -> Self {
    //     Self::ObjString(Rc::new(s.to_string()))
    // }

    //get refrence to the string data inside the Object::ObjString variant
    pub fn get_object_data(&self) -> Option<&str> {
        match self {
            Object::ObjString(obj_string) => Some(obj_string.string_rc.as_ref())
           // Some(string_rc.as_ref()),
        }
        
    }
}
