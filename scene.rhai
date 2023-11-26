let scene = Scene();

let eye = P(0.0, 0.0, 3.0);
let target = P(0.0, 0.0, 0.0);
let up = V(0.0, 1.0, 3.0);

let cam = Camera(eye, target, up, 70.0);

let material = Material(V(0.5,0.5,0.5), V(0.8, 0.8, 0.8), 25.0);

let ambient = Light(P(10.0,0.0,0.0), V(1.0,1.0,1.0), V(0.0, 0.0, 0.0));

let light2 = Light(P(0.0,0.0,10.0), V(0.0,1.0,1.0), V(0.1, 0.01, 0.001));

scene.addLight(light2);
scene.addLight(ambient);


// let sphere = Sphere(P(0.0,0.0,0.0), 1.0, material);
// let sphere_node = Node(sphere);
// scene.addNode(sphere_node);

// let cube = CubeUnit(material);
// let cube_node = Node(cube);
// scene.addNode(cube_node);

let gnonom = Gnonom(material);
let gnonom_node = Node(gnonom);
scene.addNode(gnonom_node);

// let cylinder = Cylinder(1.0,1.0, material);
// let cylinder_node = Node(cylinder);
// scene.addNode(cylinder_node);

scene