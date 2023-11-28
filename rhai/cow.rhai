
let scene = Scene();

let distance = 3.0;
let falloff = V(0.0,0.0,0.1);
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

let height = 4.0;
let spacing = 13.0;
let light = Light(P(0.0,height,spacing), V(0.0,0.3,0.3), falloff);
scene.addLight("blue", light);
let light = Light(P(0.0,height,0.0), V(0.0,0.6,0.0), falloff);
scene.addLight("green", light);
let light = Light(P(0.0,height,-spacing), V(0.3,0.0,0.0), falloff);
scene.addLight("red", light);

let material = Material(V(0.2,0.2,0.2), V(0.2, 0.8, 0.8), 10.0);
scene.addMaterial("bluegreen", material);

let mesh = Mesh("obj/cow.obj", material);
let mesh_node = Node(mesh);
scene.addNode("mesh", mesh_node);

scene