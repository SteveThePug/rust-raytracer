
let scene = Scene();

let distance = 3.0;
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

let kd = V(0.6, 0.3, 0.9); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(1.0,1.0,1.0); // Reflection color (no reflection)
let material = Material(kd, ks, kr, 10.0);
scene.addMaterial("mattee", material);

let kd = V(0.3, 0.3, 0.3); // Diffuse color (white)
let ks = V(0.7,0.7,0.7); // Specular color (no specular reflection)
let kr = V(1.0,1.0,1.0); // Reflection color (no reflection)
let material1 = Material(kd, ks, kr, 10.0);
scene.addMaterial("mattee", material1);

let kd = V(0.4, 0.0, 0.8); // Diffuse color (white)
let ks = V(0.0,0.7,0.7); // Specular color (no specular reflection)
let kr = V(1.0,1.0,1.0); // Reflection color (no reflection)
let material2 = Material(kd, ks, kr, 10.0);
scene.addMaterial("reflect", material2);

let height = 4.0;
let spacing = 4.0;

let falloff = V(0.0,0.0,0.01);

let blue = V(0.0,0.0,0.6);
let light = Light(P(0.0,height,spacing), blue, falloff);
scene.addLight("blue", light);
let green = V(0.0,0.6,0.0);
let light = Light(P(0.0,height,0.0), green, falloff);
scene.addLight("green", light);
let red = V(0.6,0.0,0.0);
let light = Light(P(0.0,height,-spacing), red, falloff);
scene.addLight("red", light);

let steiner = Steiner();
let steiner_node = Node(steiner, material2);
steiner_node.rotate(90.0,0.0,0.0);
steiner_node.translate(0.0,0.0,1.0);
 scene.addNode("steiner", steiner_node);

let steiner2 = Steiner2();
let steiner2_node = Node(steiner2, material2);
steiner2_node.active(false);
 scene.addNode("steiner2", steiner2_node);

let crosscap = CrossCap();
let crosscap_node = Node(crosscap, material);
crosscap_node.active(false);
 scene.addNode("crosscap", crosscap_node);

let p = 0.9;
let q = 0.1;
let crosscap2 = CrossCap2(p, q);
let crosscap2_node = Node(crosscap2, material);
crosscap2_node.active(true);
crosscap2_node.translate(0.0,0.0,-1.5);
crosscap2_node.rotate(140.0,0.0,90.0);
 scene.addNode("crosscap2", crosscap2_node);

let k = 2.0;
let roman = Roman(k );
let roman_node = Node(roman, material);
roman_node.active(false);
 scene.addNode("roman", roman_node);

let inner_rad = 1.0;
let outer_rad = 1.2;
let torus = Torus(inner_rad, outer_rad );
let torus_node = Node(torus, material);
torus_node.scale(0.2,0.2,0.2);
torus_node.rotate(0.0,70.0,0.0);
 scene.addNode("torus", torus_node);

scene