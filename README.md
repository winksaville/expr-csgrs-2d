# expr-csgrs-2d

csgrs doesn't seem to support operations on 2D objects.

Here is the source:
```rust
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
```

If you `cargo run` this code it generates two files `square.stl`:
```
$ cat square.stl
solid square
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 0.000000 20.000000 0.000000
      vertex 0.000000 0.000000 0.000000
      vertex 10.000000 0.000000 0.000000
    endloop
  endfacet
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 10.000000 0.000000 0.000000
      vertex 10.000000 20.000000 0.000000
      vertex 0.000000 20.000000 0.000000
    endloop
  endfacet
endsolid square
```

Adnd `square_rotated.stl`:
```
$ cat square_rotated.stl
solid square
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 0.000000 0.000000 0.000000
      vertex 0.000000 0.000000 0.000000
      vertex 10.000000 0.000000 0.000000
    endloop
  endfacet
  facet normal 0.000000 0.000000 1.000000
    outer loop
      vertex 10.000000 0.000000 0.000000
      vertex 10.000000 0.000000 0.000000
      vertex 0.000000 0.000000 0.000000
    endloop
  endfacet
endsolid square
```

The `square.stl` is two triangles in the xy plane with
the expected z-axis normal. But `square_rotated.stl` is
two degenerate triangles on the x-axis with a z-axis normal,
this is unexpected.
