use cgmath::{EuclideanVector, Vector3};

pub fn find_color(elevation: f32) -> (u8, u8, u8) {
    let water = Vector3::new(10.0, 105.0, 148.0);
    let color_ramp = [
        (-500.0,  water),
        (0.0,     water),
        (0.0,     Vector3::new(26.0, 150.0, 65.0)),
        (100.0,   Vector3::new(166.0, 217.0, 106.0)),
        (500.0,   Vector3::new(255.0, 255.0, 191.0)),
        (1000.0,  Vector3::new(253.0, 174.0, 97.0)),
        (2000.0,  Vector3::new(215.0, 25.0, 28.0)),
        (5000.0,  Vector3::new(255.0, 255.0, 255.0)),
    ];

    for i in 1..color_ramp.len() {
        let (left_height, left_color) = color_ramp[i - 1];
        let (right_height, right_color) = color_ramp[i];

        if elevation < right_height {
            let lerp_amount = (elevation - left_height) / (right_height - left_height);
            return vec3_to_rgb(&left_color.lerp(&right_color, lerp_amount));
        }
    }

    let (_, last_color) = color_ramp[color_ramp.len() - 1];
    vec3_to_rgb(&last_color)
}

fn vec3_to_rgb(vec: &Vector3<f32>) -> (u8, u8, u8) {
    let r = vec.x.floor() as u8;
    let g = vec.y.floor() as u8;
    let b = vec.z.floor() as u8;

    (r, g, b)
}
