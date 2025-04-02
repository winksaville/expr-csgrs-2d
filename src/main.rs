use csgrs::csg::CSG;

fn main() {
    let name = "square";

    // Create a 2D square
    let square: CSG<()> = CSG::square(10.0, 20.0, None);
    let stl = square.to_stl_ascii(name);
    std::fs::write(name.to_string() + ".stl", stl).unwrap();

    // Rotate the square 90 degrees around the X axis
    let square_rotated = square.rotate(90.0, 0.0, 0.0);
    let stl = square_rotated.to_stl_ascii(name);
    std::fs::write(name.to_string() + "_rotated.stl", stl).unwrap();
}