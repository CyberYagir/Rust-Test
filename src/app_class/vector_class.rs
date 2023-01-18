
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector1<T>{
    x: T
}
impl<T> Vector1<T> {
    pub fn X(&self) -> &T { return &self.x;}
}


#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector2<T> {
    x: T,
    y: T
}
impl<T> Vector2<T> {
    pub fn X(&self) -> &T { return &self.x; }
    pub fn Y(&self) -> &T {
        return &self.y;
    }
}


#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector3<T>{
    x: T,
    y: T,
    z: T
}
impl<T> Vector3<T> {
    pub fn X(&self) -> &T {
        return &self.x;
    }
    pub fn Y(&self) -> &T {
        return &self.y;
    }
    pub fn Z(&self) -> &T {
        return &self.z;
    }
}