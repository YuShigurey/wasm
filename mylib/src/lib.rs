use serde_derive::{Serialize, Deserialize};
use serde_json::{Result, json};

#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    x: f32,
    y: f32,
    x_span: f32,
    y_span: f32,
    x_max: f32,
    x_min: f32,
    y_max: f32,
    y_min: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, x_span: f32, y_span: f32) -> Rectangle{
        Rectangle{
            x,
            y,
            x_span,
            y_span,
            x_max: x+x_span/2.0,
            x_min: x-x_span/2.0,
            y_max: y+y_span/2.0,
            y_min: y-y_span/2.0,
        }
    }

    pub fn j(&self) -> Result<String> {
        let j = serde_json::to_value(&self)?;
        let ret = json!({
            "attrs": j,
            "nothing": 0,
        });
        Ok(ret.to_string())
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn rec_works(){
        let rec = Rectangle::new(1.0, 2.0, 3.0, 4.0);
        println!("{}", rec.j().unwrap());
    }
}
