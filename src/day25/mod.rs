use std::collections::HashMap;
use std::collections::HashSet;

fn input() -> String {
    String::from(
        r#"
        jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr
    "#,
    )
}

fn maximum_adjacency_search(
    g: &HashMap<String, HashMap<String, i64>>,
    start: &str,
) -> (String, String, i64) {
    let mut s = "".to_string();
    let mut t = start.to_owned();
    let mut w = 0;
    let mut candidates = HashMap::new();
    for (k, _) in g.iter() {
        if *k != *start {
            candidates.insert(
                k.to_string(),
                g.get(start).and_then(|m| m.get(k)).copied().unwrap_or(0),
            );
        }
    }
    assert!(!candidates.is_empty());
    while !candidates.is_empty() {
        let best_node = candidates
            .iter()
            .max_by_key(|&(_, v)| *v)
            .unwrap()
            .0
            .clone();
        let best_weight = candidates.remove(&best_node).unwrap();
        for (k, v) in g.get(&best_node).unwrap().iter() {
            candidates.entry(k.clone()).and_modify(|weight| {
                *weight += *v;
            });
        }
        s = t;
        t = best_node.to_string();
        w = best_weight;
    }
    (s, t, w)
}

// merge T into S
fn merge(
    mut g: HashMap<String, HashMap<String, i64>>,
    s: &str,
    t: &str,
) -> HashMap<String, HashMap<String, i64>> {
    // delete t
    let t_edges = g.remove(t).unwrap();
    // add new edges
    for (k, v) in t_edges.iter() {
        if *k != *s {
            g.entry(k.clone()).and_modify(|m| {
                m.remove(t);
                m.entry(s.to_owned())
                    .and_modify(|weight| {
                        *weight += *v;
                    })
                    .or_insert(*v);
            });
            g.entry(s.to_owned()).and_modify(|m| {
                m.remove(t);
                m.entry(k.clone())
                    .and_modify(|weight| {
                        *weight += *v;
                    })
                    .or_insert(*v);
            });
        }
    }
    g
}

fn reachable(
    g: &HashMap<String, HashMap<String, i64>>,
    merges: &HashMap<String, HashSet<String>>,
    s: &str,
    t: &str,
) -> usize {
    let mut result: HashSet<&String> = HashSet::new();
    let mut stack = vec![s.to_owned()];
    while let Some(n) = stack.pop() {
        result.extend(merges.get(&n).unwrap());
        if let Some(edges) = g.get(&n) {
            for e in edges.keys() {
                if !result.contains(e) && *e != *t {
                    stack.push(e.clone());
                }
            }
        }
    }
    result.len()
}

fn min_cut(mut g: HashMap<String, HashMap<String, i64>>) -> usize {
    let mut best_cut = i64::MAX;
    let start = g.iter().next().unwrap().0.clone();
    let mut best_partition = 0;
    let mut merges = g
        .keys()
        .map(|k| {
            let mut s = HashSet::new();
            s.insert(k.clone());
            (k.clone(), s)
        })
        .collect::<HashMap<String, _>>();
    while g.len() > 1 {
        let (s, t, w) = maximum_adjacency_search(&g, &start);
        if w < best_cut {
            best_partition = reachable(&g, &merges, &s, &t);
            best_cut = w;
        }
        g = merge(g, &s, &t);
        let t_set = merges.remove(&t).unwrap();
        merges.entry(s).and_modify(|e| {
            e.extend(t_set);
        });
    }
    best_partition
}

pub fn part1() {
    let input = input();
    let input_str = input.as_str();
    let mut graph: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for line in input_str.lines() {
        let trimmed = line.trim().replace(':', "");
        if trimmed.is_empty() {
            continue;
        }
        let parts = trimmed.split_whitespace().collect::<Vec<_>>();
        for i in 1..parts.len() {
            graph
                .entry(parts[0].to_string())
                .and_modify(|m| {
                    m.insert(parts[i].to_string(), 1);
                })
                .or_insert_with(|| {
                    let mut m = HashMap::new();
                    m.insert(parts[i].to_string(), 1);
                    m
                });
            graph
                .entry(parts[i].to_string())
                .and_modify(|m| {
                    m.insert(parts[0].to_string(), 1);
                })
                .or_insert_with(|| {
                    let mut m = HashMap::new();
                    m.insert(parts[0].to_string(), 1);
                    m
                });
        }
    }
    let nodes = graph.len();
    let min_cut = min_cut(graph);
    println!("{}", min_cut * (nodes - min_cut));
}
