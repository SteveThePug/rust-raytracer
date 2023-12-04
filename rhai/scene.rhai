let scene = Scene();

let distance = 0.99;
let camera = Camera( P(0.0,0.0,distance), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("+Z Cam", camera);
// let camera = Camera( P(0.0,distance,0.1), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
// scene.addCamera("+Y Cam", camera);
// let camera = Camera( P(distance,0.0,0.0), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
// scene.addCamera("+X Cam", camera);
// let camera = Camera( P(0.0,0.0,-distance), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
// scene.addCamera("-Z Cam", camera);
// let camera = Camera( P(0.0,-distance,0.1), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
// scene.addCamera("-Y Cam", camera);
// let camera = Camera( P(-distance,0.0,0.0), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
// scene.addCamera("-X Cam", camera);

let falloff = V(0.1, 0.1, 0.15);

let colour = V(0.0,0.5,0.5);
let pos = P(-0.5,0.9,0.5);
let light = Light(pos, colour, falloff);
light.active(true);
scene.addLight("blue", light);

let colour = V(0.0,1.0,0.0);
let pos = P(-0.5,0.9,-0.5);
let light = Light(pos, colour, falloff);
light.active(false);
scene.addLight("green", light);

let colour = V(1.0,0.0,0.0);
let light = Light(pos, colour, falloff);
light.active(false);
scene.addLight("red", light);

let colour = V(0.7,0.7,0.7);
let pos = P(0.0,0.9,0.0);
let light = Light(pos, colour, falloff);
light.active(true);
scene.addLight("white", light);

let light = Ambient(V(0.1,0.1,0.1));
light.active(true);
scene.addLight("ambient", light);

//let mesh = Mesh("obj/cow.obj" );
//let mesh_node = Node(mesh);
//scene.addNode("mesh", mesh_node);

// let child = sphere_node.child(sphere);
// child.translate(V(1.0,1.0,1.0));
//scene.addNode(child);

//let sphere2= SphereUnit();
//let sphere2_node = Node( sphere2, material2);
// sphere2_node.rotate(0.1,0.1,0.0);
// sphere2_node.translate(0.0,1.0,0.0);
//scene.addNode("sphere2",  sphere2_node);

let kd = V(1.0, 1.0, 1.0); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(0.0,0.0,0.0); // Reflection color (no reflection)
let white_wall = Material(kd, ks, kr, 10.0);
scene.addMaterial("white_wall", white_wall);

let kd = V(1.0, 0.0, 0.0); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(0.0,0.0,0.0); // Reflection color (no reflection)
let red_wall = Material(kd, ks, kr, 10.0);
scene.addMaterial("red_wall", red_wall);

let kd = V(0.0, 1.0, 0.0); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(0.0,0.0,0.0); // Reflection color (no reflection)
let green_wall = Material(kd, ks, kr, 10.0);
scene.addMaterial("green_wall", green_wall);

let kd = V(0.0, 0.0, 1.0); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(0.0,0.0,0.0); // Reflection color (no reflection)
let blue_wall = Material(kd, ks, kr, 10.0);
scene.addMaterial("blue_wall", blue_wall);


//Rear wall
let rectangle1 = RectangleUnit();
let rectangle_node1 = Node(rectangle1, white_wall);
rectangle_node1.rotate(0.0, 0.0, 0.0);
rectangle_node1.translate(0.0, 0.0, -1.0);
rectangle_node1.active(true);
scene.addNode("rectangle1", rectangle_node1);

//Behind wall
// let rectangle6 = RectangleUnit();
// let rectangle_node6 = Node(rectangle6, white_wall);
// rectangle_node6.rotate(0.0, 180.0, 0.0);
// rectangle_node6.translate(0.0, 0.0, 1.0);
// rectangle_node6.active(true);
// scene.addNode("rectangle6", rectangle_node6);

//Right wall
let rectangle2 = RectangleUnit();
let rectangle_node2 = Node(rectangle2, green_wall);
rectangle_node2.rotate(0.0, -90.0, 0.0);
rectangle_node2.translate(1.0, 0.0, 0.0);
rectangle_node2.active(true);
scene.addNode("rectangle2", rectangle_node2);

//Floor
let rectangle3 = RectangleUnit();
let rectangle_node3 = Node(rectangle3, red_wall);
rectangle_node3.rotate(0.0, 90.0, 0.0);
rectangle_node3.translate(-1.0, 0.0, 0.0);
rectangle_node3.active(true);
scene.addNode("rectangle3", rectangle_node3);

//Left wall
let rectangle4 = RectangleUnit();
let rectangle_node4 = Node(rectangle4, white_wall);
rectangle_node4.rotate(90.0, 0.0, 0.0);
rectangle_node4.translate(0.0, 1.0, 0.0);
rectangle_node4.active(true);
scene.addNode("rectangle4", rectangle_node4);

//Ceiling
let rectangle5 = RectangleUnit();
let rectangle_node5 = Node(rectangle5, white_wall);
rectangle_node5.rotate(-90.0, 0.0, 0.0);

rectangle_node5.translate(0.0, -1.0, 0.0);
rectangle_node5.active(true);
scene.addNode("rectangle5", rectangle_node5);


let kd = V(0.0, 0.0, 0.0); // Diffuse color (white)
let ks = V(0.0,0.0,0.0); // Specular color (no specular reflection)
let kr = V(1.0,1.0,1.0); // Reflection color (no reflection)
let reflective = Material(kd, ks, kr, 10.0);
scene.addMaterial("reflective", reflective);

let sphere = Sphere(P(0.0,0.0,0.0), 0.4 );
let sphere_node = Node( sphere, reflective);
sphere_node.translate(0.4, -0.6, 0.0);
scene.addNode("sphere",sphere_node);

let kd = V(0.3, 0.3, 0.3); // Diffuse color (white)
let ks = V(0.3,0.3,0.0); // Specular color (no specular reflection)
let kr = V(0.0,0.0,1.0); // Reflection color (no reflection)
let shiny = Material(kd, ks, kr, 2.0);
scene.addMaterial("shiny", shiny);

let cube = CubeUnit();
let cube_node = Node( cube, shiny);
cube_node.translate(-0.5,-0.6,0.0);
cube_node.scale(0.3,0.2,0.2);
cube_node.rotate(0.0,45.0,30.0);
scene.addNode("cube",cube_node);

let gnonom = Gnonom();
let gnonom_node = Node(gnonom, shiny);
gnonom_node.scale(0.2,0.2,0.2);
gnonom_node.translate(0.0, 0.-0.7, 0.8);
gnonom_node.rotate(0.0, 45.0, 0.0);
gnonom_node.active(false);
scene.addNode("gnonom", gnonom_node);

// let cylinder = Cylinder(2.0, 1.0);
// let cylinder_node = Node(cylinder, material);
// cylinder_node.scale(1.0, 1.0, 1.0);
// scene.addNode("cylinder", cylinder_node);

//let cone 

scene
