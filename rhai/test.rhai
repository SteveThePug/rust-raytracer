let scene = Scene();

let material = Material(V(0.2,0.2,0.2), V(0.2, 0.8, 0.8), 10.0);

//let ambient = Light(P(10.0,0.0,0.0), V(1.0,1.0,1.0), V(0.0, 0.0, 0.0));
//scene.addLight(ambient);

let light = Light(P(0.0,0.0,2.0), V(0.0,0.0,1.0), V(0.1, 0.01, 0.001));
scene.addLight("blue", light);
let light = Light(P(2.0,0.0,0.0), V(0.0,1.0,0.0), V(0.1, 0.01, 0.001));
scene.addLight("green", light);
let light = Light(P(-2.0,0.0,0.0), V(1.0,0.0,0.0), V(0.1, 0.01, 0.001));
scene.addLight("red", light);
let light = Ambient(V(0.1,0.1,0.1));
scene.addLight("ambient", light);


let stein = Steiner(material);
let stein_node = Node(stein);
 scene.addNode("stein", stein_node);

scene