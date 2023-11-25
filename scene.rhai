let scene = Scene();

let eye = P(0.0, 0.0, 3.0);
let target = P(0.0, 0.0, 0.0);
let up = V(0.0, 1.0, 3.0);
let cam = Camera(eye, target, up, 70.0, 1.0);

let material = Material(V(0.5,0.5,0.5), V(0.8, 0.8, 0.8), 25.0);

let ambient = Light(P(10.0,0.0,0.0), V(1.0,1.0,1.0), V(0.0, 0.0, 0.0));

let light2 = Light(P(0.0,0.0,10.0), V(0.0,1.0,1.0), V(0.1, 0.01, 0.001));

scene.addLight(light2);
 scene.addLight(ambient);


// let sphere = Sphere(P(0.0,0.0,0.0), 4.0, material);
// let node = Node(sphere);
// scene.addNode(node);

let stein = Stein(5.0,10.0, material);
let node = Node(stein);
scene.addNode(node);

scene