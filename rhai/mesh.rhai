let scene = Scene();

let material = Material(V(0.9,0.2,0.7), V(0.7, 0.2, 0.7), 10.0);
scene.addMaterial("material", material);
let material2 = Material(V(0.2,0.7,0.2), V(0.7, 0.7, 0.7), 10.0);
scene.addMaterial("mat2", material2);

let camera = Camera(P(0.0,0.0,2.0), P(0.0,0.0,0.0), V(0.0,1.0,0.0));
scene.addCamera("Cam", camera);

let falloff = V(0.0,0.0,0.0);
let light = Light(P(6.0,6.0,6.0), V(0.4,0.4,0.4), falloff);
light.active(false);
scene.addLight("white", light);
let light = Light(P(2.0,0.0,0.0), V(0.0,1.0,0.0), V(0.1, 0.01, 0.001));
light.active(false);
scene.addLight("green", light);
let light = Light(P(-2.0,0.0,0.0), V(1.0,0.0,0.0), V(0.1, 0.01, 0.001));
light.active(false);
scene.addLight("red", light);

let light = Ambient(V(0.3,0.3,0.3));
scene.addLight("ambient", light);

let tri = TriangleUnit();
let tri_node = Node(tri, material);
tri_node.active(false);
scene.addNode("tri", tri_node);

let circle = CircleUnit();
let circle_node = Node(circle, material);
circle_node.active(false);
scene.addNode("circle", circle_node);

let cone = ConeUnit();
let cone_node = Node(cone, material);
cone_node.active(false);
scene.addNode("cone", cone_node);

let torus = Torus(0.5, 1.5);
let torus_node = Node(torus, material);
torus_node.active(true);
scene.addNode("torus", torus_node);


let sphere = SphereUnit();
let sphere_node = Node(sphere, material);
sphere_node.translate(0.0,0.0,0.0);
sphere_node.active(false);
scene.addNode("sphere", sphere_node);

let ground = SphereUnit();
let ground_node = Node(ground, material2);
let scale = 2.0;
ground_node.translate(0.0,-scale*2.0,0.0);
ground_node.scale(scale,scale,scale);
ground_node.active(false);
scene.addNode("ground", ground_node);

let mesh = Mesh("obj/cat.obj");
let mesh_node = Node(mesh, material);
mesh_node.active(false);
scene.addNode("mesh", mesh_node);


scene