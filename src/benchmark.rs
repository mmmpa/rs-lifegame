use crate::rle::Rle;
use lifegame::game::Game;

fn work() {
    let margin = 10;

    let (w, h, map) = Rle::from_file("./fixtures/heavy.rle", margin).expect("parse INPUT error");
    let mut game = Game::new(w, h, &map);

    for _ in 0..100 {
        game.step();
    }
}

// before: test benchmark::tests::bench_work ... bench: 319,036,498 ns/iter (+/- 14,144,949)
// apply get_unchecked: test benchmark::tests::bench_work ... bench: 309,578,699 ns/iter (+/- 34,162,035)
// remove type cast: test benchmark::tests::bench_work ... bench: 303,882,741 ns/iter (+/- 18,274,540)
// apply get_unchecked_mut: test benchmark::tests::bench_work ... bench: 291,440,820 ns/iter (+/- 21,599,953)
// remove unchecked, apply clear and push: test benchmark::tests::bench_work ... bench: 291,500,261 ns/iter (+/- 8,010,288)
// remove no need clone: test benchmark::tests::bench_work ... bench: 239,832,609 ns/iter (+/- 10,374,763)
#[cfg(test)]
mod tests {
    use test::Bencher;
    use crate::benchmark::work;

    #[bench]
    fn bench_work(b: &mut Bencher) {
        b.iter(|| work());
    }
}
