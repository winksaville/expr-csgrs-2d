# expr-csgrs-2d

csgrs doesn't seem to support operations on 2D objects.

Here is the source:
```rust
use csgrs::csg::CSG;

fn main() {
    let font_data = include_bytes!("../fonts/courier-prime-sans/courier-prime-sans.ttf").to_vec();
    let name = "hello";

    // Creaee a 2D text object with "hello"
    let hello: CSG<()> = CSG::text("hello", &font_data, 10.0, None);
    let stl = hello.to_stl_ascii(name);
    std::fs::write(name.to_string() + ".stl", stl).unwrap();

    // Rotate the text object 90 degrees around the x-axis
    let hello_rotated = hello.rotate(90.0, 0.0, 0.0);
    let stl = hello_rotated.to_stl_ascii(name);
    std::fs::write(name.to_string() + "_rotated.stl", stl).unwrap();
}
```

If your `cargo run` this code it generates two files `hello.stl`
and `hello_rotated.stl` look at `hello.stl` you'll see the text
"hello" on the xy plane as would be expected. But if you look at
`hello_rotated.stl` it is series of degenerate triangles forming
a line the x-axis.

```
solid hello
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 0.584576 0.000000 0.000000
      vertex 0.577915 0.000000 0.000000
      vertex 0.570150 0.000000 0.000000
    endloop
  endfacet
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 0.570150 0.000000 0.000000
      vertex 0.561066 0.000000 0.000000
      vertex 0.550663 0.000000 0.000000
    endloop
  endfacet
..
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 8.094250 0.000000 0.000000
      vertex 7.827888 0.000000 0.000000
      vertex 7.828978 0.000000 0.000000
    endloop
  endfacet
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 7.828978 0.000000 0.000000
      vertex 7.827888 0.000000 0.000000
      vertex 8.094250 0.000000 0.000000
    endloop
  endfacet
endsolid hello
```
