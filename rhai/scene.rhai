let scene = Scene();

let distance = 10.0;
let camera = Camera( P(0.0,0.0,distance), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("+Z Cam", camera);
let camera = Camera( P(0.0,distance,0.1), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("+Y Cam", camera);
let camera = Camera( P(distance,0.0,0.0), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("+X Cam", camera);
let camera = Camera( P(0.0,0.0,-distance), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("-Z Cam", camera);
let camera = Camera( P(0.0,-distance,0.1), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("-Y Cam", camera);
let camera = Camera( P(-distance,0.0,0.0), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("-X Cam", camera);

let material = Material(V(0.2,0.2,0.2), V(0.2, 0.8, 0.8), 10.0);
scene.addMaterial("bluegreen", material);

// let light = Light(P(0.0,7.0,0.0), V(0.0,0.0,1.0), V(0.1, 0.01, 0.001));
// scene.addLight("blue", light);

// let light = Light( P(2.0,7.0,0.0), V(0.0,1.0,0.0), V(0.1, 0.01, 0.001));
// scene.addLight("green", light);

let light = Light( P(-2.0,7.0,0.0), V(1.0,0.0,0.0), V(0.1, 0.01, 0.001));
scene.addLight("red", light);

let light = Ambient(V(0.1,0.1,0.1));
scene.addLight("ambient", light);


let sphere = Sphere(P(0.0,0.0,0.0), 1.0, material);
let sphere_node = Node(sphere);
 scene.addNode("sphere", sphere_node);

//let mesh = Mesh("obj/cow.obj", material);
//let mesh_node = Node(mesh);
//scene.addNode("mesh", mesh_node);

for i in 0..6 {
    let sphere = Sphere(P(0.0,0.0,0.0), 2.0, material);
     let sphere_node = Node(sphere);
     sphere_node.translate(2.0*cos(i.to_float()), -4.0, 2.0*sin(i.to_float()));
     scene.addNode(i.to_string(), sphere_node);
 }
// let child = sphere_node.child(sphere);
// child.translate(V(1.0,1.0,1.0));
//scene.addNode(child);

let cube = CubeUnit(material);
let cube_node = Node(cube);
scene.addNode("cube", cube_node);

//let gnonom = Gnonom(material);
//let gnonom_node = Node(gnonom);
//scene.addNode("gnonom", gnonom_node);

//let cylinder = Cylinder(2.0,1.0, material);
//let cylinder_node = Node(cylinder);
//cylinder_node.scale(1.0,1.0,1.0);
//scene.addNode("cylinder",cylinder_node);

scene