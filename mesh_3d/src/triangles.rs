use std::fs::File;
use std::io::{BufWriter, Write};
use std::alloc;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Default for Point {
    fn default() -> Self {
        Point {
            x: 0 as f32,
            y: 0 as f32,
            z: 0 as f32
        }
    }
}

type Triangle = [Point; 3];

struct Mesh {
    cap: usize,
    size: usize,
    pov: Point,
    data: *mut Triangle,
}

impl Mesh {
    fn new(cap: usize, pov: Point) -> Mesh {
        Self {
            cap,
            size: 0,
            pov,
            data: alloc_triangle(cap),
        }
    }

    fn resize(&mut self) {
        let new_cap = self.cap * 2;
        let new_data = alloc_triangle(new_cap);

        for i in 0..self.size {
            unsafe { *new_data.add(i) = *self.data.add(i) }
        }

        self.cap = new_cap;
        self.data = new_data;
    }

    fn change_pov(&mut self, new_pov: Point) {
        self.pov = new_pov;
        self.min_heapify(self.size - 1, self.pov);
    }

    fn push(&mut self, elem: Triangle) {
        if self.size == self.cap { self.resize(); }

        unsafe { *self.data.add(self.size) = elem; }
        self.size += 1;

        self.min_heapify(self.size - 1, self.pov);
    }

    fn pop_min(&mut self) -> Option<Triangle> {
        if self.size == 0 { return None; }
        let min: Triangle = self.at_idx(0);
        let last = self.at_idx(self.size - 1);
        self.size -= 1;

        unsafe { *self.data.add(0) = last; }
        self.min_heapify(0, self.pov);
        
        Some(min)
    }

    fn swap(&mut self, idx: usize, other: usize) {
        if idx > self.size - 1 || other > self.size - 1 { return; }
        unsafe {
            let tmp = *self.data.add(idx);
            *self.data.add(idx) = *self.data.add(other);
            *self.data.add(other) = tmp;
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn at_idx(&self, idx: usize) -> Triangle {
        unsafe { *self.data.add(idx) }
    }

    fn min_heapify(&mut self, idx: usize, p: Point) {
        let l = left(idx);
        let r = right(idx);

        let mut small_val = distance_pt(p, self.at_idx(idx));
        let mut small_idx = idx;

        if l < self.len() && distance_pt(p, self.at_idx(l)) < small_val {
            small_val = distance_pt(p, self.at_idx(l));
            small_idx = l;
            
        }
        if r < self.len() && distance_pt(p, self.at_idx(r)) < small_val {
            small_idx = r;
        }

        if idx != small_idx {
            self.swap(idx, small_idx);

            self.min_heapify(small_idx, p);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        dealloc_triangle(self.data, self.cap);
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        let new_data = alloc_triangle(self.cap);

        for i in 0..self.size {
            unsafe { *new_data.add(i) = *self.data.add(i); }
        }
        Mesh {
            cap: self.cap,
            size: self.size,
            pov: self.pov,
            data: new_data 
        }
    }
}

fn left(idx: usize) -> usize { idx * 2 + 1 }
fn right(idx: usize) -> usize { idx * 2 + 2 }


//---------------------
// Allocation

fn alloc_triangle(count: usize) -> *mut Triangle {
    let layout = alloc::Layout::array::<Triangle>(count).unwrap();
    unsafe { alloc::alloc(layout) as *mut Triangle }
}

fn dealloc_triangle(ptr: *mut Triangle, count: usize) {
    let layout = alloc::Layout::array::<Triangle>(count).unwrap();
    unsafe { alloc::dealloc(ptr as *mut u8, layout) }
}

//--------------------



fn distance_pp(p1: Point, p2: Point) -> f32 {
    ((p1.x - p2.x).powi(2)
        + (p1.y - p2.y).powi(2)
        + (p1.z - p2.z).powi(2))
        .sqrt()
}

fn distance_pt(p: Point, t: Triangle) -> f32 {
    let d0 = distance_pp(p, t[0]);
    let d1 = distance_pp(p, t[1]);
    let d2 = distance_pp(p, t[2]);
    d0.min(d1).min(d2)
}


fn load_mesh(filename: &str) -> Mesh {
    let contents = std::fs::read_to_string(filename)
        .unwrap_or_else(|error| panic!("failed to read mesh file '{filename}': {error}"));
    let mut values = contents.split_whitespace();
    let triangle_count = values
        .next()
        .expect("mesh file is missing its triangle count")
        .parse::<usize>()
        .unwrap_or_else(|error| panic!("invalid triangle count in '{filename}': {error}"));
    
    let mut mesh = Mesh::new(triangle_count, Point::default());

    
    for _ in 0..triangle_count {
        let mut tmp_points= Vec::new();
        for _ in 0..3 {
            let x = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let y = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");
            let z = values
                .next()
                .expect("mesh file is missing a vertex coordinate")
                .parse::<f32>()
                .expect("mesh vertex coordinate is not a valid number");  
            tmp_points.push(Point {x, y, z});
        }

        let trianlge: Triangle = [tmp_points[0], tmp_points[1], tmp_points[2]];
        mesh.push(trianlge);
    }
    mesh
}

fn free_mesh(_mesh: &mut Mesh) {
    // TODO
    //mesh.triangles.clear();
}

fn save_to_file(mesh: &Mesh, filename: &str) {
    let file = File::create(filename)
        .unwrap_or_else(|error| panic!("failed to create mesh file '{filename}': {error}"));
    let mut writer = BufWriter::new(file);

    let mut new_mesh = mesh.clone();

    writeln!(writer, "{}", mesh.len()).expect("failed to write triangle count");
    loop {
        let Some(point) = new_mesh.pop_min() else {
            break;
        };
        writeln!(
            writer,
            "{} {} {} {} {} {} {} {} {}",
            point[0].x, point[0].y, point[0].z,
            point[1].x, point[1].y, point[1].z,
            point[2].x, point[2].y, point[2].z,)
                .expect("failed to write vertex coordinates");
    }
}

fn closest_triangles(mesh: &Mesh, p: Point, count: usize) -> Mesh {
    let mut tmp_mesh = mesh.clone();
    tmp_mesh.change_pov(p);

    let mut return_mesh = Mesh::new(count, p);
    loop {
        match tmp_mesh.pop_min() {
            Some(triangle) => return_mesh.push(triangle),
            None => break,
        }
        if return_mesh.len() >= count {
            break;
        }
    }
    return_mesh
}

pub fn main() {
    let mesh = load_mesh("bunny.txt");
    save_to_file(&mesh, "test_bunny_copy.txt");

    let players = [
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Point {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
    ];
    let counts = [1, 5, 10, 100, 1000, 10000, 100000];

    for (player_index, player) in players.iter().enumerate() {
        for count in counts {
            println!("Player {}, Count {}", player_index + 1, count);
            let mut closest = closest_triangles(&mesh, *player, count);
            println!("Closest gotten!");
            let filename = format!("player_{}_count_{}.mesh", player_index + 1, count);
            save_to_file(&closest, &filename);
            free_mesh(&mut closest);
        }
    }
}
