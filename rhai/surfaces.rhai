
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
let spacing = 4.0;
let blue = V(0.0,0.0,0.6);
let light = Light(P(0.0,height,spacing), blue, falloff);
scene.addLight("blue", light);
let green = V(0.0,0.6,0.0);
let light = Light(P(0.0,height,0.0), green, falloff);
scene.addLight("green", light);
let red = V(0.6,0.0,0.0);
let light = Light(P(0.0,height,-spacing), red, falloff);
scene.addLight("red", light);

let material = Material(V(0.2,0.2,0.2), V(0.2, 0.8, 0.8), 10.0);
scene.addMaterial("bluegreen", material);

let steiner = Steiner();
let steiner_node = Node(steiner, material);
 scene.addNode("steiner", steiner_node);

let steiner2 = Steiner2();
let steiner2_node = Node(steiner2, material2);
 scene.addNode("steiner2", steiner2_node);

let crosscap = CrossCap();
let crosscap_node = Node(crosscap, material);
 scene.addNode("crosscap", crosscap_node);

let p = 1.0;
let q = 1.0;
let crosscap2 = CrossCap2(p, q);
let crosscap2_node = Node(crosscap2, material);
 scene.addNode("crosscap2", crosscap2_node);

let k = 0.5;
let roman = Roman(k );
let roman_node = Node(roman, material);
 scene.addNode("roman", roman_node);

let inner_rad = 1.0;
let outer_rad = 0.5;
let torus = Torus(inner_rad, outer_rad );
let torus_node = Node(torus, material);
 scene.addNode("torus", torus_node);

scene