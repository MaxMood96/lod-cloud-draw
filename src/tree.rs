use graph::Graph;
use std::f64::consts::PI;
use std::collections::HashSet;

pub fn build_tree(graph : &Graph, radius : f64) -> Vec<f64> {
    let mut nodes = HashSet::new();
    nodes.extend(0..graph.n);

    let mut edges = Vec::new();
    edges.resize(graph.n, Vec::new());

    for edge in graph.edges.iter() {
        edges[edge.src].push(edge.trg);
    }

    let mut v0 = 0;
    let mut max_edges = 0;
    for i in 0..graph.n {
        if edges[i].len() > max_edges {
            v0 = i;
            max_edges = edges[i].len();
        }
    }

    nodes.remove(&v0);

    let mut loc = Vec::new();
    loc.resize(graph.n * 2, 0.0f64);
    
    calculate_loc(&mut loc, v0, &mut nodes, &edges, graph, radius);

    loc
}

// Convert an (x,y) coordinate to an angle where (1,0) => 0
fn to_angle(x : f64, y : f64) -> f64 {
    if x > 0.0 {
        if y > 0.0 {
            (x/y).atan()
        } else if y < 0.0 {
            2.0 * PI + (x/y).atan()
        } else {
            0.0
        }
    } else if x < 0.0 {
        if y > 0.0 {
            PI + (y/x).atan()
        } else if y < 0.0 {
            PI + (y/x).atan()
        } else {
            PI
        }
    } else {
        if y > 0.0 {
            PI / 2.0
        } else if y < 0.0 {
            3.0 * PI / 2.0
        } else {
            0.0
        }
    }
}




fn calculate_loc(loc : &mut Vec<f64>, parent : usize, nodes : &mut HashSet<usize>,
                 edges : &Vec<Vec<usize>>, graph : &Graph, radius : f64) {

    let children : Vec<&usize> = edges[parent].iter().
        filter(|x| nodes.contains(x)).collect();

    let astep = if loc[parent * 2] == 0.0 && loc[parent * 2 + 1] == 0.0 {
        2.0 * PI / (children.len() as f64)
    } else {
        PI / ((children.len() + 1) as f64)
    };

    let ainitial = to_angle(loc[parent * 2], loc[parent * 2 + 1]) - PI / 2.0;

    for child in children.iter() {
        nodes.remove(child);
    }

    let mut a = 1.0;
    for &child in children.iter() {
        loc[child * 2] = loc[parent * 2] + radius * (ainitial + a * astep).cos();
        loc[child * 2 + 1] = loc[parent * 2 + 1] + radius * (ainitial + a * astep).sin();

        calculate_loc(loc, *child, nodes, edges, graph, radius);
        a += 1.0;
    } 

}

#[cfg(test)]
mod tests {
    use graph::{Graph, Edge};
    use tree::{build_tree, to_angle};
    use std::f64::consts::PI;

    #[test]
    fn test_angle() {
        assert!((to_angle(1.0,0.0) - 0.0) < 1e-4);
        assert!((to_angle(0.0,1.0) - PI / 2.0) < 1e-4);
        assert!((to_angle(-1.0,0.0) - PI) < 1e-4);
        assert!((to_angle(0.0,-1.0) - 3.0 * PI / 2.0) < 1e-4);
        assert!((to_angle(1.0,1.0) - PI / 4.0) < 1e-4); 
        assert!((to_angle(-1.0,1.0) - 3.0 * PI / 4.0) < 1e-4); 
        assert!((to_angle(-1.0,-1.0) - 5.0 * PI / 4.0) < 1e-4); 
        assert!((to_angle(1.0,-1.0) - 7.0 * PI / 4.0) < 1e-4); 
        //assert!((to_angle(10.0,0.0) - 0.0) < 1e-4);
        //assert!((to_angle(-0.000000001,-10.0) - 3.0 * PI / 2.0) < 1e-4);
        //assert!((to_angle(0.000000001,-10.0) - 3.0 * PI / 2.0) < 1e-4);
        //assert!((to_angle(-1.0,0.00001) - PI) < 1e-4);
        //assert!((to_angle(-1.0,-0.000001) - PI) < 1e-4);
    }


    #[test]
    fn test_tree() {
        let mut g = Graph::new();
        let v1 = g.add_vertex();
        let v2 = g.add_vertex();
        let v3 = g.add_vertex();
        let v4 = g.add_vertex();
        let v5 = g.add_vertex();
        g.edges.push(Edge::new(v1, v2));
        g.edges.push(Edge::new(v1, v3));
        g.edges.push(Edge::new(v1, v4));
        g.edges.push(Edge::new(v4, v5));

        let result = build_tree(&g, 10.0);

        let exp = [0.0,0.0,
                   10.0 * (PI / 6.0).cos(), 10.0 * (PI/6.0).sin(),
                   -10.0 * (PI / 6.0).cos(), 10.0 * (PI/6.0).sin(),
                   0.0,-10.0,
                   0.0,-20.0
        ];

        for i in 0..exp.len() {
            println!("{}: {:.3} == {:.3}",i, exp[i], result[i]);
            assert!((exp[i] - result[i]).abs() < 1e-4);
        } 
    }
}
