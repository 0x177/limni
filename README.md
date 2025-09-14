# limni!
Lua bindings for my library [thimni](https://codeberg.org/0x177/thimni).

## usage
While thimni is n-dimensional, the lua bindings are currently restricted to 2D
and 3D. Support for n-dimensional SDFs in the lua bindings will be implemented
when the bindings are more stable and mature.

Limni will work with any table containing the following two functions:
1. `dist`:
  - inputs: `self` and a point
  - outputs: the signed distance between that point and the surface of the
  shape.
2. `get_aabb`:
  - inputs: `self`
  - outputs: A table with two elements, `min` and `max`. Both of them are points.

Points and vectors are represented by a table containing `x`, `y`, and
optionally `z`.

The collision heavily depends on the collision parameters, which are
represented as a table whose elements share the same data types and names as
the [rust counterpart](https://docs.rs/thimni/latest/thimni/utils/struct.CollisionParameters.html).

The collision functions are the following, with N replaced with 2 or 3:
1. `get_collision_Nd`:
  - inputs: collision parameters, an SDF, an SDF
  - outputs: a collision result table containing a `point` and a `gradient`.
2. `approximate_depth_Nd`:
  - inputs: collision parameters, and SDF, and SDF, and the result of the collision
  - outputs: approximated depth of the collision between the two SDFs

## TODO
| Issue                                | urgency       |
| :----------------------------------: | :-----------: |
| add raycasting                       | high          |
| add helper for combinations of SDFs  | high          |
| add 3D examples                      | medium        |
| add support for N-dimensional SDFs   | low           |
