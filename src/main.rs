mod graph;
mod traversal;

fn main() {
    let mut g = graph::Graph::new();

    // People
    let thomas_gouveia = g.add_v()
        .property("age", "22")
        .property("firstName", "Thomas")
        .property("lastName", "Gouveia")
        .build();

    let christopher_nolan = g.add_v()
        .property("age", "52")
        .property("firstName", "Christopher")
        .property("lastName", "Nolan")
        .build();

    let matt_damon = g.add_v()
        .property("job", "actor")
        .property("firstName", "Matt")
        .property("lastName", "Damon")
        .build();

    let jessica_chastain = g.add_v()
        .property("job", "actor")
        .property("firstName", "Jessica")
        .property("lastName", "Chastain")
        .build();

    // Movies
    let interstellar = g.add_v()
        .property("title", "Interstellar")
        .property("releaseDate", "2014")
        .property("type", "movie")
        .build();

    let inception = g.add_v()
        .property("title", "Inception")
        .property("releaseDate", "2010")
        .property("type", "movie")
        .build();

    let oscar_best_visual_effects = g.add_v()
        .property("distinction", "Oscar of the bests visual effects")
        .build();

    // Add edges
    g.add_e("directed")
        .source(&christopher_nolan)
        .destination(&interstellar)
        .build();

    g.add_e("directed")
        .source(&christopher_nolan)
        .destination(&inception)
        .build();

    g.add_e("acted_in")
        .source(&matt_damon)
        .destination(&interstellar)
        .build();

    g.add_e("acted_in")
        .source(&matt_damon)
        .destination(&inception);

    g.add_e("acted_in")
        .source(&jessica_chastain)
        .destination(&interstellar)
        .build();

    g.add_e("like")
        .source(&thomas_gouveia)
        .destination(&interstellar)
        .build();

    g.add_e("acquired")
        .source(&inception)
        .destination(&oscar_best_visual_effects)
        .build();

    // We can now play with the graph

    section("Get all vertices of the graph");
    for vertex in g.v(None).execute() {
        println!("{}", vertex);
    }

    section("Get all edges of the graph");
    for edge in g.e() {
        println!("{}", edge);
    }

    section("Get all vertices with attribute type=movie");
    for vertex in g.v(None).has("type", "movie").execute() {
        println!("{}", vertex);
    }

    section("Get all in edges for movies");
    for edge in g.v(None).has("type", "movie").in_e() {
        println!("{}", edge);
    }

    section("Get all out edges for movies");
    for edge in g.v(None).has("type", "movie").out_e() {
        println!("{}", edge);
    }

    section("Get all in vertices for movies");
    for vertex in g.v(None).has("type", "movie").r#in().execute() {
        println!("{}", vertex);
    }

    section("Get all out vertices for movies");
    for vertex in g.v(None).has("type", "movie").out().execute() {
        println!("{}", vertex);
    }
}

fn section(label: &str) {
    println!();
    println!("============================================");
    println!("{}", label);
    println!("============================================");
}