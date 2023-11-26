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


let sphere = Stein(material);
let sphere_node = Node(sphere);
 scene.addNode(sphere_node);

scene