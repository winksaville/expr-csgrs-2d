use csgrs::csg::CSG;

fn main() {
    let font_data = include_bytes!("../fonts/courier-prime-sans/courier-prime-sans.ttf").to_vec();
    let name = "hello";

    // Creaee a 2D text object with "hello"
    let hello: CSG<()> = CSG::text("hello", &font_data, 10.0, None);
    let stl = hello.to_stl_ascii(name);
    std::fs::write(name.to_string() + ".stl", stl).unwrap();

    // Rotate the text object 90 degrees around the X axis
    let hello_rotated = hello.rotate(90.0, 0.0, 0.0);
    let stl = hello_rotated.to_stl_ascii(name);
    std::fs::write(name.to_string() + "_rotated.stl", stl).unwrap();
}