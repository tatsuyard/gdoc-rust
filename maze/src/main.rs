use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();
    let mut maze = [[0; MAP_N]; MAP_N];
    for n in 0..MAP_N {
        maze[n][0] = 1;
    }
}
