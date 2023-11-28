
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

let steiner = Steiner( material);
let steiner_node = Node(steiner);
 scene.addNode("steiner", steiner_node);

let steiner2 = Steiner2( material);
let steiner2_node = Node(steiner2);
 scene.addNode("steiner2", steiner2_node);

let crosscap = CrossCap(material);
let crosscap_node = Node(crosscap);
 scene.addNode("crosscap", crosscap_node);

let (p,q) = (1.0,1.0);
let crosscap2 = CrossCap2( material);
let crosscap2_node = Node(crosscap2);
 scene.addNode("crosscap2", crosscap2_node);

let k = 1.0;
let roman = Roman(k, material);
let roman_node = Node(roman);
 scene.addNode("sphere", roman_node);

let (inner_rad, outer_rad) = (0.5, 0.5);
let torus = Torus(inner_rad, outer_rad, material);
let torus_node = Node(torus);
 scene.addNode("sphere", torus_node);

scene