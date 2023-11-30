use crate::{node::Node, ray::*, EPSILON};
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;
use std::ops::Index;

// BOUNDING BOX -----------------------------------------------------------------
#[derive(Clone)]
pub struct AABB {
    pub bln: Point3<f64>,
    pub trf: Point3<f64>,
    pub centroid: Point3<f64>,
}

impl AABB {
    // New box with respective coordinates
    pub fn new(bln: Point3<f64>, trf: Point3<f64>) -> AABB {
        let bln = bln + Vector3::new(EPSILON, EPSILON, EPSILON);
        let trf = trf - Vector3::new(EPSILON, EPSILON, EPSILON);
        let centroid = bln + (bln - trf) / 2.0;
        AABB { bln, trf, centroid }
    }
    pub fn empty() -> AABB {
        AABB {
            bln: Point3::new(0.0, 0.0, 0.0),
            trf: Point3::new(0.0, 0.0, 0.0),
            centroid: Point3::new(0.0, 0.0, 0.0),
        }
    }
    // Intersect bounding box exactly
    pub fn intersect_bounding_box(&self, ray: &Ray) -> bool {
        let bln = &self.bln;
        let trf = &self.trf;
        let t1 = (bln - ray.a).component_div(&ray.b);
        let t2 = (trf - ray.a).component_div(&ray.b);

        let tmin = t1.inf(&t2).min();
        let tmax = t1.sup(&t2).max();

        if tmax >= tmin {
            let intersect = ray.at_t(tmin);

            // Check if the intersection is inside the box
            if intersect.x > bln.x
                || intersect.x < trf.x
                || intersect.y > bln.y
                || intersect.y < trf.y
                || intersect.z > bln.z
                || intersect.z < trf.z
            {
                return true; // Intersection is outside the box
            }
        }
        false
    }
    // Intersect way with some epsilon term
    pub fn intersect_bounding_box_aprox(&self, ray: &Ray) -> bool {
        let bln = &self.bln;
        let trf = &self.trf;
        let t1 = (bln - ray.a).component_div(&ray.b);
        let t2 = (trf - ray.a).component_div(&ray.b);

        let tmin = t1.inf(&t2).min();
        let tmax = t1.sup(&t2).max();

        if tmax >= tmin {
            let intersect = ray.at_t(tmin);

            // Check if the intersection is inside the box
            if intersect.x > bln.x - EPSILON
                || intersect.x < trf.x + EPSILON
                || intersect.y > bln.y - EPSILON
                || intersect.y < trf.y + EPSILON
                || intersect.z > bln.z - EPSILON
                || intersect.z < trf.z + EPSILON
            {
                return true; // Intersection is outside the box
            }
        }
        false
    }
    // Get the center of this bounding box
    fn get_centroid(&self) -> Point3<f64> {
        self.centroid
    }
    // Make a new AABB that contains both
    pub fn join(&self, other: &AABB) -> AABB {
        AABB::new(
            Point3::new(
                self.bln.x.min(other.bln.x),
                self.bln.y.min(other.bln.y),
                self.bln.z.min(other.bln.z),
            ),
            Point3::new(
                self.trf.x.max(other.trf.x),
                self.trf.y.max(other.trf.y),
                self.trf.z.max(other.trf.z),
            ),
        )
    }
    //Join mutably
    pub fn join_mut(&mut self, other: &AABB) {
        self.bln = Point3::new(
            self.bln.x.min(other.bln.x),
            self.bln.y.min(other.bln.y),
            self.bln.z.min(other.bln.z),
        );
        self.trf = Point3::new(
            self.trf.x.max(other.trf.x),
            self.trf.y.max(other.trf.y),
            self.trf.z.max(other.trf.z),
        );
    }
    //Grow the AABB to contain the cover the point
    pub fn grow(&self, other: &Point3<f64>) -> AABB {
        AABB::new(
            Point3::new(
                self.bln.x.min(other.x),
                self.bln.y.min(other.y),
                self.bln.z.min(other.z),
            ),
            Point3::new(
                self.trf.x.max(other.x),
                self.trf.y.max(other.y),
                self.trf.z.max(other.z),
            ),
        )
    }
    //Grow mutably
    pub fn grow_mut(&mut self, other: &Point3<f64>) {
        self.bln = Point3::new(
            self.bln.x.min(other.x),
            self.bln.y.min(other.y),
            self.bln.z.min(other.z),
        );
        self.trf = Point3::new(
            self.trf.x.max(other.x),
            self.trf.y.max(other.y),
            self.trf.z.max(other.z),
        );
    }
    // Size of AABB
    pub fn size(&self) -> Vector3<f64> {
        self.trf - self.bln
    } //Surface area of AABB
    pub fn surface_area(&self) -> f64 {
        let size = self.size();
        2.0 * (size.x * size.y + size.x * size.z + size.y * size.z)
    }
    // Volume of the AABB
    pub fn volume(&self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }
}

// Index implemntation of the BVH tree
// pub enum BVHNode {
//     Leaf {
//         p_idx: usize, //Parent index
//         depth: usize, //Depth in BVH tree
//         n_idx: usize, //Node index in corrosponding Vec<Node>
//     },
//     Node {
//         p_idx: usize, //Parent index
//         l_idx: usize, //Left child index
//         l_aabb: AABB, //Left AABB
//         r_idx: usize, //Right child index
//         r_aabb: AABB, //Right AABB
//         depth: usize, //Depth in BVH tree
//     },
// }
// impl BVHNode {
//     //Get parent
//     fn get_parent(&self) -> usize {
//         match *self {
//             BVHNode::Node { p_idx, .. } | BVHNode::Leaf { p_idx, .. } => p_idx,
//         }
//     }
//     //Get the left child of a node
//     fn get_child_l(&self) -> usize {
//         match *self {
//             BVHNode::Leaf { .. } => panic!("Cannot get child of leaf node"),
//             BVHNode::Node { l_idx, .. } => l_idx,
//         }
//     }
//     // Get right child
//     fn get_child_r(&self) -> usize {
//         match *self {
//             BVHNode::Leaf { .. } => panic!("Cannot get child of leaf node"),
//             BVHNode::Node { r_idx, .. } => r_idx,
//         }
//     }
//     // Get the depth of selected node
//     pub fn depth(&self) -> usize {
//         match *self {
//             BVHNode::Node { depth, .. } | BVHNode::Leaf { depth, .. } => depth,
//         }
//     }
//     // Get the aabb of the current node, if leaf return the primitives aabb
//     // If node return the join of the two child nodes
//     pub fn get_node_aabb(&self, nodes: &Vec<Node>) -> AABB {
//         match *self {
//             BVHNode::Node { l_aabb, r_aabb, .. } => l_aabb.join(&r_aabb),
//             BVHNode::Leaf { aabb, .. } => aabb,
//         }
//     }
// }
// //Implementation of the BVH
// pub struct BVHTree<'a> {
//     pub nodes: &'a HashMap<String, Node>,
//     pub bvh_nodes: Vec<BVHNode>,
// }

// impl<'a> BVHTree<'a> {
//     //Generate a BVH tree given a vector of nodes
//     pub fn new(nodes: &HashMap<String, Node>) -> BVHTree {
//         //We will make an aabb that bounds all shapes
//         let mut root_aabb = AABB::empty();
//         let mut root_centroid = AABB::empty();
//         for (_, node) in nodes {
//             let node_aabb = node.primitive.get_aabb();
//             root_aabb.join_mut(&node_aabb);
//             root_centroid.grow_mut(&node_aabb.get_centroid());
//         }

//         //We will make an aabb that bounds all centroids
//         return BVHTree {
//             nodes: &HashMap::new(),
//             bvh_nodes: vec![],
//         };
//     }
// }

pub struct BVHNode {
    aabb: AABB,        //The nodes bounding box
    l_idx: usize,      //Child node l, the right node is alway l_idx + 1
    first_prim: usize, //First primitive that the node encapsulates
    prim_count: usize, //Number of primitives the node encapsulates
}

pub struct BVH {
    bvh_nodes: Vec<BVHNode>, //BVH nodes with AABBs
    nodes: Vec<Node>,        //Nodes with primitives
    nodes_used: usize,
    root_node_index: usize,
}

impl BVH {
    //Build a bvh by subdividing recursively
    fn build(in_nodes: HashMap<String, Node>) -> BVH {
        //Make our own vec of nodes so that we can refer to it by index
        //Might be long to copy scene, so alternative methods may be prefered
        let nodes = vec![];
        for (_, node) in in_nodes {
            nodes.push(node);
        }

        //A BVH tree will be maximum size of 2*n + 1
        let n = nodes.len();
        let mut bvh_nodes: Vec<BVHNode> = Vec::with_capacity(2 * n + 1);

        //Begin constructing our BVH tree
        let root_node_index = 0;
        let nodes_used = 1;
        let tree = BVH {
            nodes,
            bvh_nodes,
            root_node_index,
            nodes_used,
        };

        // Get the root node and assign it to index 0
        let mut root = &bvh_nodes[root_node_index];
        root.l_idx = 0; //Root node has no children to begin with
        (root.first_prim, root.prim_count) = (0, n); //Make root include all n nodes
        tree.update_bvh_node_aabb(root_node_index); //Fit the root nodes AABB
        tree.subdivide(root_node_index);
        tree
    }
    // Will update the node's AABB at bvh[index]
    fn update_bvh_node_aabb(&mut self, index: usize) {
        // We will make his node bound all its primitives
        let bvh_node = &self.bvh_nodes[index]; //Get the BVHNode we are working
        let bvh_node_aabb = AABB::empty(); //Create the BVHNode's AABB

        let start_index = bvh_node.first_prim; //Start index of the first primitive the node contains
        let count = bvh_node.prim_count; //Number of primitives within the nodes aabb

        for i in 0..count {
            let primitive = &self.nodes[start_index + i].primitive; //Get the primitive from the Vec<Node>
            let node_aabb = primitive.get_aabb(); //Get the primitives aabb
            bvh_node_aabb.join_mut(&node_aabb); //Join it with the bvh_nodes aabb
        }
    }

    fn subdivide(&mut self, index: usize) {
        // Determine the axis and position of the split plane
        // Split the group of primitives in two halves using the split plane
        // Create child nodes for each half
        // Recurse into each of the child nodes.

        // Get information about the node we want to subdivide
        let bvh_node = &self.bvh_nodes[index]; //Get the BVHNode we are working

        /* ----------------- SUBDIVIDE BY CENTROID --------------------- */
        // let bvh_node_centroid_aabb = AABB::empty(); //Create the BVHNode's AABB
        // let start_index = bvh_node.first_prim; //Start index of the first primitive the node contains
        // let count = bvh_node.prim_count; //Number of primitives within the nodes aabb
        // for i in 0..count {
        //     let primitive = &self.nodes[start_index + i].primitive; //Get the primitive from the Vec<Node>
        //     let node_aabb_centroid = primitive.get_aabb().get_centroid(); //Get the primitives aabb centroid
        //     bvh_node_centroid_aabb.grow_mut(&node_aabb_centroid); // Grow the aabb to include the all centroids
        // }

        /* ------------ SUBDIVIDE BY LONGEST AXIS ------------ */

        let (bln, trf) = (bvh_node.aabb.bln, bvh_node.aabb.trf);
        let extent = trf - bln;
        let axis = 0; // Assume that x is longest
        if extent.y > extent.x {
            axis = 1 // Split y if longer
        };
        if extent.z > extent[axis] {
            axis = 2 // Split z if loner
        };
        let split_pos = bln[axis] + extent[axis] * 0.5; //Final split along this axis

        //Perform a quicksort our nodes
        let i = bvh_node.first_prim;
        let j = i + bvh_node.prim_count - 1;
        while i <= j {
            let centroid = self.nodes[i].primitive.get_aabb().get_centroid();
            if centroid[axis] < split_pos {
                i += 1; //If it is on left split remain in place
            } else {
                self.nodes.swap(i, j); //Move to right split
                j -= 1;
            }
        }
        //Now we have two children, the lhs of the array is in the left split, and the rhs of the array is on the right split
        let left_count = i - bvh_node.first_prim; //Number of prims on lhs
        if left_count == 0 || left_count == bvh_node.prim_count {
            return; //If we have no more on the left, disregard
        }
        let l_idx = self.nodes_used; //Left child
        self.nodes_used += 1;
        let r_idx = self.nodes_used; //Right child
        self.nodes_used += 1;

        bvh_node.l_idx = l_idx;

        self.bvh_nodes[l_idx].first_prim = bvh_node.first_prim; //Set left split
        self.bvh_nodes[l_idx].prim_count = left_count; //We know this info from our quicksort

        self.bvh_nodes[r_idx].first_prim = i; //Set right split information
        self.bvh_nodes[r_idx].prim_count = bvh_node.prim_count - left_count;
        bvh_node.prim_count = 0;

        self.update_bvh_node_aabb(l_idx); //Update AABB for left of split
        self.update_bvh_node_aabb(r_idx); //Update AABB for right of split

        //Recurse
        self.subdivide(l_idx);
        self.subdivide(r_idx);
    }
}
