let scene = Scene();

let material = Material(V(0.2,0.2,0.2), V(0.2, 0.8, 0.8), 10.0);

//let ambient = Light(P(10.0,0.0,0.0), V(1.0,1.0,1.0), V(0.0, 0.0, 0.0));
//scene.addLight(ambient);

let light = Light(P(0.0,7.0,0.0), V(0.0,0.0,1.0), V(0.1, 0.01, 0.001));
scene.addLight(light);
let light = Light(P(2.0,7.0,0.0), V(0.0,1.0,0.0), V(0.1, 0.01, 0.001));
scene.addLight(light);
let light = Light(P(-2.0,7.0,0.0), V(1.0,0.0,0.0), V(0.1, 0.01, 0.001));
scene.addLight(light);
let light = Ambient(V(0.1,0.1,0.1));
scene.addLight(light);


let sphere = Sphere(P(0.0,0.0,0.0), 1.0, material);
let sphere_node = Node(sphere);
 scene.addNode(sphere_node);

for i in 0..6 {
    let sphere = Sphere(P(0.0,0.0,0.0), 2.0, material);
    let sphere_node = Node(sphere);
    sphere_node.translate(V(2.0*cos(i.to_float()), -4.0, 2.0*sin(i.to_float())));
    scene.addNode(sphere_node);
}
// let child = sphere_node.child(sphere);
// child.translate(V(1.0,1.0,1.0));
//scene.addNode(child);

// let cube = CubeUnit(material);
// let cube_node = Node(cube);
// scene.addNode(cube_node);

// let gnonom = Gnonom(material);
// let gnonom_node = Node(gnonom);
// scene.addNode(gnonom_node);

// let cylinder = Cylinder(10.0,1.0, material);
// let cylinder_node = Node(cylinder);
// //cylinder_node.scale(V(4.0,0.3,0.3));
// cylinder_node.translate(V(0.0,-4.0,0.0));
// scene.addNode(cylinder_node);

scene