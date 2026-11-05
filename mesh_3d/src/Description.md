# Task 2 - 3D Mesh 

## Background

In 3D computer graphics, objects are often represented by collection of (connected) triangles. 
When rendering these (with their textures), the engine has to make clever decisions to filter out the triangles that the camera can actually see, which is then followed by a plenty of matrix multiplications.

Our goal is to provide a very simple solution for this first filtering step.

## Main Goal

The main task is to provide an implementation for this function: 

```Rust
fn closest_triangles(mesh: &Mesh, p: Point, count: usize) -> Mesh
```

Where:
 - `mesh` contains all the triangles. 
 - `p` is the point where the viewer is.
 - `count` is the number of triangles our GPU can draw on the screen to keep a 120 Hz refresh rate. 
 - The function should return a subset of the `count` closest triangles to the viewer. 
 - In order to keep the desired refresh rate, our filtering algorithm cannot run slower than $O(n\log{n})$, where $n$ is the number of triangles in `mesh`.

## Technical details

### Example data

The example data is in [bunny.txt](bunny.txt) :

```
144046
-0.630100 1.499700 -0.541100 -0.620500 1.505400 -0.542300 -0.621000 1.497800 -0.541700 
-0.621000 1.497800 -0.541700 -0.620500 1.505400 -0.542300 -0.615700 1.499800 -0.538700 
-0.621000 1.497800 -0.541700 -0.615700 1.499800 -0.538700 -0.615400 1.491800 -0.537400 
-0.615400 1.491800 -0.537400 -0.615700 1.499800 -0.538700 -0.614100 1.498600 -0.532500 
-0.615400 1.491800 -0.537400 -0.614100 1.498600 -0.532500 -0.613800 1.489000 -0.528000 
```

The first line contains the number of triangles, then each line contains the coordinates of the 3 vertices of a triangle in `x1 y1 z1 x2 y2 z2 x3 y3 z3` format.

In the `main` function (that is not allowed to be changed) the closest 1, 5, 10, 100, 1000, 10000 and 10000 triangles are filtered for two viewpoints: $(1,1,1)$ and $(-1,-1,-1)$. 
The expected outputs are also provided, which will be overwritten by main, so save them somewhere first if you want to use them for comparing with your results.

### Data types

The types for `Point` and `Triangle` are provided, designing `Mesh` is your task.


```Rust
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

type Triangle = [Point; 3];


struct Mesh {
    // TODO
}
```

If you want to change these types (e.g. changing `Triangle` to a `struct`), feel free to do so, just pay attention to the functions that are alredy implemented for them.


The skeletons of the file parsing and exporting functions for `Mesh` are provided, but they needs to be completed.

Very simplistic distance functions are also provided.

The main simply loads the bunny, then for all viewpoints and all `count` values filters out the triangles and saves them in `player_{player-number}_count_{count}.mesh`.

## Bonus exercise

Don't waste time on unnecessarily sorting triangles that are too far.

