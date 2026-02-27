use serde::{Deserialize, Serialize};

pub type Vector = [f32; 2];

#[derive(Serialize, Deserialize)]
pub struct SplineGeometry(pub [[f32; 4]; 4], pub Vec<Vector>);

#[derive(Serialize, Deserialize)]
pub struct AutonSegment {
    pub spline: SplineGeometry,
}

#[derive(Serialize, Deserialize)]
pub struct Auton(pub Vec<AutonSegment>);
