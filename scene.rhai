let scene = Scene();

let eye = P(0.0, 0.0, 3.0);
let target = P(0.0, 0.0, 0.0);
let up = V(0.0, 1.0, 3.0);
let cam = Camera(eye, target, up, 70.0, 1.0);

let material = Material(V(0.5,0.5,0.5), V(0.8, 0.8, 0.8), 0.5);

let light2 = Light(P(0.0,0.0,-10.0), V(0.8, 0.8, 0.8), V(0.2, 0.0, 0.0));

scene.addLight(light2);


// let sphere = Sphere(P(0.0,0.0,0.0), 4.0, material);
// let node = Node(sphere);
// scene.addNode(node);

let stein = Torus(1.0,2.0, material);
let node = Node(stein);
scene.addNode(node);

scene