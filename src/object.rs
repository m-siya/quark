use std::rc::Rc;

#[derive(Clone)]
pub enum Object{
    ObjString(Rc<String>),

}

impl Object{
    pub fn from_str(s: &str) -> Self {
        Self::ObjString(Rc::new(s.to_string()))
    }

    //get refrence to the string data inside the Object::ObjString variant
    pub fn get_object_data(&self) -> Option<&str> {
        match self {
            Object::ObjString(string_rc) => Some(string_rc.as_ref()),
        }
        
    }
}

// pub struct Object {
//     object_type: ObjectType,
// }

// pub struct ObjectString<'a> {
//     object: Object,
//     length: i32,
//     chars: &'a str,
// }