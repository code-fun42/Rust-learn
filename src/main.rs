struct Cache;

trait Has {
    fn has_key(&self) -> bool;
}

impl Has for Cache {
    fn has_key(&self) -> bool {
        false
    }
}

trait Default {
    fn default() -> Self;
}

fn make_default<T: Default>() -> T {
    T::default()
}

fn default_pair<T: Default>() -> (T, T) {
    (make_default(), make_default())
}

struct Circle;

impl Default for Circle {
    fn default() -> Circle {
        Circle {}
    }
}

fn main() {
    let _: (Circle, Circle) = default_pair();
    let circle: Circle = make_default::<Circle>();
}
