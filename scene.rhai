// engine
//     .register_type::<Vector3<f32>>()
//     .register_fn("V", Vector3::<f32>::new);
// engine
//     .register_type::<Point3<f32>>()
//     .register_fn("P", Point3::<f32>::new);
// engine
//     .register_type::<Scene>()
//     .register_fn("Scene", Scene::empty)
//     .register_fn("addNode", Scene::add_node)
//     .register_fn("addLight", Scene::add_light);

// engine
//     .register_type::<Node>()
//     .register_fn("Node", Node::new)
//     .register_fn("translate", Node::translate)
//     .register_fn("rotate", Node::rotate)
//     .register_fn("scale", Node::scale);
// engine
//     .register_type::<Camera>()
//     .register_fn("Camera", Camera::new);
// engine
//     .register_type::<Light>()
//     .register_fn("Light", Light::new);
// engine
//     .register_type::<Material>()
//     .register_fn("Material", Material::new)
//     .register_fn("MaterialRed", Material::red)
//     .register_fn("MaterialBlue", Material::blue)
//     .register_fn("MaterialGreen", Material::green)
//     .register_fn("MaterialMagenta", Material::magenta)
//     .register_fn("MaterialTurquoise", Material::turquoise);
// engine
//     .register_type::<Sphere>()
//     .register_fn("Sphere", Sphere::new)
//     .register_fn("SphereUnit", Sphere::unit);
// engine
//     .register_type::<Cube>()
//     .register_fn("Cube", Cube::new)
//     .register_fn("CubeUnit", Cube::unit);
// engine
//     .register_type::<Cone>()
//     .register_fn("Cone", Cone::new)
//     .register_fn("ConeUnit", Cone::unit);
// engine
//     .register_type::<Cylinder>()
//     .register_fn("Cylinder", Cylinder::new);
// engine
//     .register_type::<Circle>()
//     .register_fn("Circle", Circle::new)
//     .register_fn("CircleUnit", Circle::unit);
// engine
//     .register_type::<Rectangle>()
//     .register_fn("Rectangle", Rectangle::new)
//     .register_fn("RectangleUnit", Rectangle::unit);
// engine
//     .register_type::<SteinerSurface>()
//     .register_fn("Steiner", SteinerSurface::new);
// engine
//     .register_type::<Torus>()
//     .register_fn("Torus", Torus::new);

let scene = Scene();

let eye = P(0.0, 0.0, 3.0);
let target = P(0.0, 0.0, 0.0);
let up = V(0.0, 1.0, 3.0);
let cam = Camera(eye, target, up, 70.0, 1.0);

let material = Material(V(0.5,0.5,0.5), V(0.8, 0.8, 0.8), 25.0);

let ambient = Light(P(10.0,0.0,0.0), V(1.0,1.0,1.0), V(0.0, 0.0, 0.0));

let light2 = Light(P(0.0,0.0,10.0), V(0.0,1.0,1.0), V(0.1, 0.01, 0.001));

scene.addLight(light2);
 scene.addLight(ambient);


// let sphere = Sphere(P(0.0,0.0,0.0), 4.0, material);
// let node = Node(sphere);
// scene.addNode(node);

let stein = Steiner(material);
let node = Node(stein);
scene.addNode(node);

scene