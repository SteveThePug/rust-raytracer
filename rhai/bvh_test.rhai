// BVH Stress Test Scene - Grid of spheres
let scene = Scene();

// Camera
let camera = Camera(P(0.0, 2.0, 5.0), P(0.0, 0.0, 0.0), V(0.0, 1.0, 0.0));
scene.addCamera("main", camera);

// Lights
let light = Light(P(3.0, 5.0, 3.0), V(0.8, 0.8, 0.8), V(0.05, 0.05, 0.05));
light.active(true);
scene.addLight("key", light);

let light2 = Light(P(-3.0, 4.0, -2.0), V(0.4, 0.4, 0.6), V(0.05, 0.05, 0.05));
light2.active(true);
scene.addLight("fill", light2);

let ambient = Ambient(V(0.1, 0.1, 0.1));
ambient.active(true);
scene.addLight("ambient", ambient);

// Materials
let kd = V(0.8, 0.2, 0.2);
let ks = V(0.3, 0.3, 0.3);
let kr = V(0.0, 0.0, 0.0);
let red = Material(kd, ks, kr, 10.0);
scene.addMaterial("red", red);

let kd = V(0.2, 0.2, 0.8);
let ks = V(0.3, 0.3, 0.3);
let kr = V(0.0, 0.0, 0.0);
let blue = Material(kd, ks, kr, 10.0);
scene.addMaterial("blue", blue);

let kd = V(0.2, 0.8, 0.2);
let ks = V(0.3, 0.3, 0.3);
let kr = V(0.0, 0.0, 0.0);
let green = Material(kd, ks, kr, 10.0);
scene.addMaterial("green", green);

let kd = V(0.8, 0.8, 0.2);
let ks = V(0.3, 0.3, 0.3);
let kr = V(0.0, 0.0, 0.0);
let yellow = Material(kd, ks, kr, 10.0);
scene.addMaterial("yellow", yellow);

let kd = V(0.8, 0.4, 0.8);
let ks = V(0.3, 0.3, 0.3);
let kr = V(0.1, 0.1, 0.1);
let purple = Material(kd, ks, kr, 15.0);
scene.addMaterial("purple", purple);

// Floor
let floor = RectangleUnit();
let floor_node = Node(floor, green);
floor_node.rotate(-90.0, 0.0, 0.0);
floor_node.translate(0.0, -1.5, 0.0);
floor_node.scale(5.0, 5.0, 1.0);
scene.addNode("floor", floor_node);

// Grid of spheres: 5 x 4 x 5 = 100 spheres
let materials = [red, blue, green, yellow, purple];
let count = 0;

for x in range(-2, 3) {
    for y in range(0, 4) {
        for z in range(-2, 3) {
            let mat_idx = count % 5;
            let mat = materials[mat_idx];
            let sphere = Sphere(P(0.0, 0.0, 0.0), 0.15);
            let node = Node(sphere, mat);
            let px = x.to_float() * 0.7;
            let py = y.to_float() * 0.7 - 1.0;
            let pz = z.to_float() * 0.7 - 1.0;
            node.translate(px, py, pz);
            scene.addNode("s" + count, node);
            count += 1;
        }
    }
}

scene
