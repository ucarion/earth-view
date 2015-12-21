use image::{self, GenericImage};

#[derive(Debug)]
pub struct Quadtree<T> {
    top_left: (f32, f32),
    side_length: f32,
    data: [T; 4],
    children: Option<[Box<Quadtree<T>>; 4]>
}

impl<T> Quadtree<T> {
    /// Constructs a quadtree from a grid.
    ///
    /// `grid_side_length` must equal 2^n + 1 for some integer n.
    pub fn from_grid(data: &[T], side_length: usize) -> Quadtree<T> {
        Self::from_grid_range(data, 0, side_length)
    }

    fn from_grid_range(data: &[T], min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Quadtree<T> {
        if base_case {

        } else {
            let data = [
                data[
            ];
            // make a, b, c, d from min/max x/y
            Quadtree {
                top_left: (min_x, min_y),
                side_length: max_x - min_x,
                data: [a, b, c, d],
            }
        }
    }
}

pub fn main() {
    let color_path = "color_chunks/left.jpg";
    let color_data = image::open(color_path).unwrap();

    let side_length = 1025;

    let mut data = Vec::with_capacity(side_length * side_length);
    for y in 0..side_length {
        for x in 0..side_length {
            let color = color_data.get_pixel(x as u32, y as u32);
            data.push(color.data);
        }
    }

    let quadtree = Quadtree::from_grid(&data, side_length);
    println!("{:?}", quadtree);
}
