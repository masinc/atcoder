use proconio::input;
use rustc_hash::FxHashMap;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Object {
    weight: usize,
    price: usize,
}

impl From<(usize, usize)> for Object {
    fn from(x: (usize, usize)) -> Self {
        Object {
            weight: x.0,
            price: x.1,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Key {
    index: usize,
    weight: usize,
}

impl Key {
    fn new(index: usize, weight: usize) -> Self {
        Self { index, weight }
    }

    fn next_index_key(&self) -> Self {
        Self {
            index: self.index + 1,
            weight: self.weight,
        }
    }
}

fn main() {
    input! {
        n: usize,
        w: usize,
        o: [(usize, usize); n]
    };

    let mut memo: FxHashMap<Key, usize> = FxHashMap::default();

    // init
    memo.insert(Key::new(0, 0), 0);

    // compute weight
    for (i, obj) in o
        .into_iter()
        .enumerate()
        .map(|(i, tpl)| (i + 1, Object::from(tpl)))
    {
        for wi in 0..w {
            let prev_index_key = Key::new(i - 1, wi);

            if let Some(prev_index_price) = memo.get(&prev_index_key).cloned() {
                let key = prev_index_key.next_index_key();
                if let Some(price) = memo.get_mut(&key) {
                    *price = usize::max(*price, prev_index_price)
                } else {
                    memo.insert(key, prev_index_price);
                }

                let new_weight = wi + obj.weight;
                if new_weight > w {
                    continue;
                }

                let key = Key::new(i, wi + obj.weight);
                memo.insert(key, obj.price + prev_index_price);
            }
        }
    }

    let r = memo.values().max().unwrap();
    println!("{}", r);
}
