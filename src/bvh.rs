use crate::{node::Node, ray::*, EPSILON};
use nalgebra::{distance, point, Matrix4, Point3, Vector3};
use std::collections::HashMap;
use std::fmt;

// Debuging statics
static mut STATIC0: i32 = 0;
static mut STATIC1: i32 = 0;
static mut STATIC2: i32 = 0;
static mut STATIC3: i32 = 0;
static mut STATIC4: i32 = 0;

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
        let centroid = bln + (trf - bln) / 2.0;
        AABB { bln, trf, centroid }
    }
    //Empty box
    pub fn empty() -> AABB {
        AABB {
            bln: Point3::new(f64::MAX, f64::MAX, f64::MAX),
            trf: Point3::new(f64::MIN, f64::MIN, f64::MIN),
            centroid: Point3::new(0.0, 0.0, 0.0),
        }
    }
    //Apply a matrix transformation to a box
    pub fn transform_mut(&mut self, mat: &Matrix4<f64>) {
        let bln = &mut self.bln;
        let trf = &mut self.trf;
        let centroid = &mut self.centroid;
        self.bln = mat.transform_point(bln);
        self.trf = mat.transform_point(trf);
        self.centroid = mat.transform_point(centroid);
    }
    // Intersect bounding box exactly
    pub fn intersect_ray(&self, ray: &Ray) -> bool {
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
    pub fn intersect_ray_aprox(&self, ray: &Ray) -> bool {
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
    pub fn area(&self) -> f64 {
        let extent = self.trf - self.bln;
        return extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
    }
    // Volume of the AABB
    pub fn volume(&self) -> f64 {
        let size = self.size();
        size.x * size.y * size.z
    }
}
impl fmt::Display for AABB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bln[0] == f64::MAX || self.trf[0] == f64::MIN {
            writeln!(f, "Empty aabb")
        } else {
            writeln!(f, "bln: {}\ntrf: {}", self.bln, self.trf)
        }
    }
}
#[derive(Clone)]
pub struct BVHNode {
    aabb: AABB,        //The nodes bounding box
    l_idx: usize,      //Child node l, the right node is alway l_idx + 1
    first_prim: usize, //First primitive that the node encapsulates
    prim_count: usize, //Number of primitives the node encapsulates
}

impl BVHNode {
    pub fn default() -> BVHNode {
        BVHNode {
            aabb: AABB::empty(),
            l_idx: 0,
            first_prim: 0,
            prim_count: 0,
        }
    }
}

impl fmt::Display for BVHNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "l_idx: {}", self.l_idx)?;
        writeln!(f, "First Prim: {}", self.first_prim)?;
        writeln!(f, "Prim Count: {}", self.prim_count)?;
        writeln!(f, "aabb: {}", self.aabb)
    }
}

pub struct BVH {
    bvh_nodes: Vec<BVHNode>, //BVH nodes with AABBs
    nodes: Vec<Node>,        //Nodes with primitives
    nodes_used: usize,
}

impl BVH {
    //Build a bvh by subdividing recursively
    pub fn build(in_nodes: &HashMap<String, Node>) -> BVH {
        /*
        Make our own vec of nodes so that we can refer to it by index
        This might be expensive so another method is preferred
        */
        let mut nodes = vec![];
        for (_, node) in in_nodes {
            nodes.push(node.clone());
        }

        //A BVH tree will be maximum size of 2*n + 1
        //Initialise an empty BVHNode with empty AABB
        let n = nodes.len();
        let bvh_nodes: Vec<BVHNode> = vec![BVHNode::default(); 2 * n + 1];

        //Begin constructing our BVH tree
        //One node used to begin with (The root node)
        let nodes_used = 1;
        let mut tree = BVH {
            nodes,
            bvh_nodes,
            nodes_used,
        };
        // Get the root node at index 0
        let root = &mut tree.bvh_nodes[0];
        root.l_idx = 0; //Root node has no left or right child to begin
        (root.first_prim, root.prim_count) = (0, n); //Make root include all n nodes
        tree.update_bvh_node_aabb(0); //Create the root nodes AABB on the n primitives
        tree.subdivide(0); //Sub divide the root node
        tree
    }
    // Will update the node's AABB at bvh_nodes[index]
    fn update_bvh_node_aabb(&mut self, index: usize) {
        // We will make his node bound all its primitives
        let bvh_node = &mut self.bvh_nodes[index]; // Current BVHNode
        let bvh_node_aabb = &mut bvh_node.aabb; //Current node AABB

        let first_prim = bvh_node.first_prim; //Start index of prim
        let prim_count = bvh_node.prim_count; //Number of primitives within the nodes aabb

        for i in 0..prim_count {
            let node = &self.nodes[first_prim + i]; //Get the node from the Vec<Node>
            bvh_node_aabb.join_mut(&node.aabb); //Join it with the BVH node's AABB
        }

        // unsafe {
        //     println!("UPDATE TO AABB ---- {STATIC0}");
        //     STATIC0 += 1;
        //     let bvh_node = &mut self.bvh_nodes[index]; //Get the BVHNode we are working on
        //     println!("{bvh_node}");
        // }
    }
    // Subdivision, will subdivide a split
    fn subdivide(&mut self, index: usize) {
        //Get the bvh_node we will be altering
        // Determine the axis and position of the split plane
        // Split the group of primitives in two halves using the split plane
        // Create child nodes for each half
        // Recurse into each of the child nodes.

        //Leaf node case, we cannot sub-divide any more
        if self.bvh_nodes[index].prim_count == 1 {
            return;
        };

        /* ------------ SUBDIVIDE BY LONGEST AXIS ------------ */
        //Get information about the node we want to subdivide
        let (bln, trf) = (
            self.bvh_nodes[index].aabb.bln,
            self.bvh_nodes[index].aabb.trf,
        );
        let extent = trf - bln;
        let mut axis = 0; // Assume that x is longest
        if extent.y > extent.x {
            axis = 1; // Split y if longest
        };
        if extent.z > extent[axis] {
            axis = 2; // Split z if longest
        };
        let split_pos = bln[axis] + extent[axis] * 0.5; // Final split down the middle of AABB

        /* --------- SUBDIVIDE BY Surface Area Heuristic ---------*/
        // let mut best_axis: Option<usize> = None;
        // let mut best_pos = 0.0;
        // let mut best_cost = 1e30;
        // let first_prim_idx = self.bvh_nodes[index].first_prim;
        // for axis in 0..2 {
        //     for i in 0..self.bvh_nodes[index].prim_count {
        //         let node = &self.nodes[first_prim_idx + i];
        //         //Get the centroid of the bounding box
        //         let centroid = node.aabb.get_centroid();
        //         //Get the candidate position
        //         let candidate_pos = world_centroid[axis];
        //         let cost = self.evaluate_sah(&self.bvh_nodes[index], axis, candidate_pos);
        //         if cost < best_cost {
        //             best_pos = candidate_pos;
        //             best_axis = Some(axis);
        //             best_cost = cost;
        //         }
        //     }
        // }
        // let axis = match best_axis {
        //     Some(axis) => axis,
        //     None => 0,
        // };
        // let split_pos = best_pos;

        let left_count;
        let right_count;
        let mut i;
        let mut j;
        {
            let bvh_node = &mut self.bvh_nodes[index];
            i = bvh_node.first_prim; //Start of array
            j = i + bvh_node.prim_count - 1; //End of array
            while i <= j {
                //Perform a quicksort dependent on location
                let node = &self.nodes[i]; // Node we would like to sort
                let centroid = node.aabb.get_centroid(); //Centroid of node we would like to sort
                if centroid[axis] < split_pos {
                    i += 1; // On Left-Hand-Side
                } else {
                    self.nodes.swap(i, j);
                    j -= 1; // On Right-Hand-Side
                }
            }
            //Now we have two splits
            //The lhs of the array is in the left split  0..left_count
            //The rhs of the array is on the right split left_count + 1..n
            left_count = i - bvh_node.first_prim; //Number of prims on lhs
            right_count = bvh_node.prim_count - left_count;
            //println!("SPLIT INTO: {left_count} {right_count}");
            if left_count == 0 || left_count == bvh_node.prim_count {
                //Split did nothing
                return;
            }
        }
        // unsafe {
        //     println!("SUBDIVIDE: {STATIC1}");
        //     println!("SPLIT INTO: {left_count} ");
        //     STATIC1 += 1;
        // }

        let l_idx = self.nodes_used; //Left child
        self.bvh_nodes[index].l_idx = l_idx;
        self.nodes_used = self.nodes_used + 2;

        //Set left node information
        self.bvh_nodes[l_idx].first_prim = self.bvh_nodes[index].first_prim; //Left split begins at parent split
        self.bvh_nodes[l_idx].prim_count = left_count; // Left prims

        //Set right node information
        self.bvh_nodes[l_idx + 1].first_prim = i; // Right split start index
        self.bvh_nodes[l_idx + 1].prim_count = right_count;

        //Current node is not a leaf node
        self.bvh_nodes[index].prim_count = 0;

        self.update_bvh_node_aabb(l_idx); //Update AABB for left of split
        self.update_bvh_node_aabb(l_idx + 1); //Update AABB for right of split

        //Recurse
        self.subdivide(l_idx); // Subdivide left index
        self.subdivide(l_idx + 1); // SUbdivide right index
    }
    // Traverse the BVH, 0 will be needed to start at root node
    pub fn traverse(&self, ray: &Ray, idx: usize) -> Option<(&Node, Intersection)> {
        let bvh_node = &self.bvh_nodes[idx];
        if !bvh_node.aabb.intersect_ray(ray) {
            // No intersection with BVH in world coordinates
            return None;
        }
        if bvh_node.prim_count > 0 {
            // Leaf node intersection
            let node_idx = bvh_node.first_prim;
            let node = &self.nodes[node_idx];
            if !node.active {
                return None;
            }
            let ray = ray.transform(&node.inv_model); //Transform ray to model coords
            if let Some(intersect) = node.primitive.intersect_ray(&ray) {
                if intersect.distance < EPSILON {
                    return None;
                } else {
                    // Convert intersect back to world coords
                    let intersect = intersect.transform(&node.model, &node.inv_model);
                    return Some((node, intersect));
                }
            }
            return None;
        } else {
            //Recurse down the BVH
            //Recurse down the BVH right node
            let intersect_l = self.traverse(ray, bvh_node.l_idx);
            let intersect_r = self.traverse(ray, bvh_node.l_idx + 1);

            match (intersect_l, intersect_r) {
                (None, None) => return None,
                (Some(intersect), None) => return Some(intersect),
                (None, Some(intersect)) => return Some(intersect),
                (Some((node_l, inter_l)), Some((node_r, inter_r))) => {
                    //Compare intersect distance
                    let dist_l = distance(&ray.a, &inter_l.point);
                    let dist_r = distance(&ray.a, &inter_r.point);
                    if dist_l < dist_r {
                        return Some((node_l, inter_l));
                    } else {
                        return Some((node_r, inter_r));
                    }
                }
            }
        }
    }
    fn evaluate_sah(&self, node: &BVHNode, axis: usize, pos: f64) -> f64 {
        // determine triangle counts and bounds for this split candidate
        let mut l_aabb = AABB::empty();
        let mut r_aabb = AABB::empty();
        let mut l_count = 0;
        let mut r_count = 0;
        for i in 0..node.prim_count {
            let aabb = self.nodes[node.first_prim + i].primitive.get_aabb();
            if aabb.trf[axis] < pos {
                l_count += 1;
                l_aabb.grow_mut(&aabb.trf);
            } else {
                r_count += 1;
                r_aabb.grow_mut(&aabb.bln);
            }
        }
        let cost = l_count as f64 * l_aabb.area() + r_count as f64 * r_aabb.area();
        match cost > 0.0 {
            true => 0.0,
            false => 1e30,
        }
    }
}

impl fmt::Display for BVH {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, node) in self.bvh_nodes.iter().enumerate() {
            writeln!(f, "Node: {i}")?;
            writeln!(f, "{node}")?;
        }
        write!(f, "")
    }
}
