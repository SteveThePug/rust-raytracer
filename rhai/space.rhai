let scene = Scene();

let falloff = V(0.0,0.0,0.0);

//CAMERAS
let camera = Camera( P(100.0,100.0,100.0), P(500.0,500.0,500.0), V(0.0,1.0,0.0));
scene.addCamera("Main Camera", camera);

//Light for the sun
let light = Light(P(800.0, 800.0, 250.0), V(1.0, 1.0, 0.929), falloff);
scene.addLight("Sun", light);

//Ball for the sun 
let material = Material(V(1.0, 1.0, 0.9), V(0.9, 0.9, 0.9), 10.0);
scene.addMaterial("material_sun", material);
let sphere = Sphere(P(800.0, 800.0, 200.0), 50.0);
let sphere_node = Node(sphere, material);
 scene.addNode("sphere", sphere_node);

//Ball for the planet 
let material = Material(V(0.2,0.8,0.2), V(0.2, 0.8, 0.8), 10.0);
scene.addMaterial("material_planet", material);
let sphere = Sphere(P(500.0, 500.0, 500.0), 50.0);
let sphere_node = Node(sphere, material);
 scene.addNode("sphere", sphere_node);


// let sphere = Steiner( material);
// let sphere_node = Node(sphere);
//  scene.addNode("sphere", sphere_node);

scene