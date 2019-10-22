#[macro_use]
extern crate criterion;
extern crate fudd;

use criterion::black_box;
use criterion::Criterion;

use fudd::get_fudd;

pub fn criterion_benchmark(c: &mut Criterion) {
    let rabbit_fire_elmer_script =
        "Shh. Be very, very quiet. I'm hunting rabbits. (laughs) Oh boy, rabbit tracks.
     Now I got you. You, you, rabbit.
     It's not?
     Hey you, come back here!
     Well what do you know. No more bullets.
     Well what do you know. One bullet left
     I'm sorry fellas but I'm a vegetarian. I just hunt for the sport of it.
         Now you screwy wabbit, your next!
     Come on out or I'll blast you out!
     Elephant gun!
     (angry) Ooh, just wait til I get that screwy rabbit and that screwy daffy duck!
     Hey! What's the big idea? Why don't you look where you'r (doesn't get to finish)
     Aw, shucks. Well, I (blushes and laughs)
     Okay rabbit, I see though that disguise. Say your prayers!
     You too duck!
     Uh oh!";

    c.bench_function("I like hunting rabbits", |b| {
        b.iter(|| get_fudd(black_box(rabbit_fire_elmer_script)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
