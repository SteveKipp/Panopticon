use iced::{ Vector, };

pub struct Grid {
    scaling: f32,
    translation: Vector
}

//STOPNOTE - implement the maxs, min for lat long
// looks like there are some functions that can be implemented here
// too - for for creating points, and the applying the canvas widget to 
// the grid struct -- good ol' IMPL
impl Grid {
    const MAX_LAT: f32 = 90.0;
    const MIN_LAT: f32 = -90.0;

    const MAX_LONG: f32 = 180.0;
    const MIN_LONG: f32 = -180.0;

    pub fn build() -> Self{
        //here Self = Grid
        Self {
            scaling: 0.01,
            translation: Vector::default(),
        }
    }
}