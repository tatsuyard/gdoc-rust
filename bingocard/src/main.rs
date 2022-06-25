use rand::seq::SliceRandom;

fn main() {
    let mut nums = [0; 75];
    for i in ..=75 {
        nums[i - 1] = i;
    }
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);
}
