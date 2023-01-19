
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector1<T> {
    pub x: T
}
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Vector3<T>{
    pub x: T,
    pub y: T,
    pub z: T
}