#![allow(dead_code)]

#[test]
fn test_sort_integers() {
    let mut x = [1, 4, 3, 2];
    x.sort();
    assert!(x == [1, 2, 3, 4]);
}

#[test]
fn test_sort_struct() {
    struct City {
        name: String,
        population: i64,
        country: String,
    }
    #[derive(Debug, Copy, Clone)]
    struct Statistic {}
    impl City {
        fn get_statistic(&self, _stat: Statistic) -> i64 {
            return 1;
        }
    }
    fn city_population_descending(city: &City) -> i64 {
        -city.population
    }
    fn sort_cities(cities: &mut Vec<City>) {
        cities.sort_by_key(city_population_descending);
    }
    fn sort_cities2(cities: &mut Vec<City>) {
        cities.sort_by_key(|city| -city.population);
    }
    /// Sort by any of several different statistics.
    fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
        cities.sort_by_key(|city| -city.get_statistic(stat));
    }
    use std::thread;
    fn start_sorting_thread(
        mut cities: Vec<City>,
        stat: Statistic,
    ) -> thread::JoinHandle<Vec<City>> {
        let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };
        thread::spawn(move || {
            cities.sort_by_key(key_fn);
            cities
        })
    }

    fn count_selected_cities<F>(cities: &Vec<City>, test_fn: F) -> usize
    where
        F: Fn(&City) -> bool,
    {
        let mut count = 0;
        for city in cities {
            if test_fn(city) {
                count += 1
            }
        }
        count
    }
    let my_cities = vec![];
    count_selected_cities(&my_cities, |city| city.name == "hello");
}

#[test]
fn test_kill() {
    let my_str = "hello".to_string();
    let f = || drop(my_str);
    f();
    //f();
}

use std::collections::HashMap;

fn call_twice<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}

#[test]
fn test_once() {
    fn call_twice<F>(closure: F)
    where
        F: Fn(),
    {
        closure();
        closure();
    }
    let _my_str = "hello".to_string();
    //let f = || drop(my_str);
    //call_twice(f);
}

#[test]
fn test_mut() {
    let mut i = 0;
    let incr = || {
        i += 1; // incr borrows a mut reference to i
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);
    let mut i = 0;
    call_twice(|| i += 1); // ok!
    assert_eq!(i, 2);
}

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }
    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }
}
impl BasicRouter {
    fn handle_request(&self, request: &Request) -> Response {
        match self.routes.get(&request.url) {
            None => not_found_response(),
            Some(callback) => callback(request),
        }
    }
}

fn not_found_response() -> Response {
    Response {
        code: 404,
        headers: HashMap::new(),
        body: vec![],
    }
}

fn get_form_response() -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

fn get_gcd_response(_req: &Request) -> Response {
    Response {
        code: 200,
        headers: HashMap::new(),
        body: vec![],
    }
}

#[test]
fn test_router() {
    let mut router = BasicRouter::new();
    router.add_route("/", |_| get_form_response());
    router.add_route("/gcd", |req| get_gcd_response(req));
}

fn main() {
    println!("Hello, world!");
}

// Chapter 14. Closures -> Callbacks, page 316
