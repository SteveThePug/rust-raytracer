let scene = Scene();

let eye = P(0.0, 0.0, 3.0);
let target = P(0.0, 0.0, 0.0);
let up = V(0.0, 1.0, 3.0);
let cam = Camera(eye, target, up, 70.0, 1.0);

let material = Material(V(0.5,0.5,0.5), V(0.8, 0.1, 0.2), 0.25);

let sphere = Sphere(P(0.0,0.0,0.0), 0.5, material);

let node = Node(sphere);
scene.addNode(node);
scene.render();
