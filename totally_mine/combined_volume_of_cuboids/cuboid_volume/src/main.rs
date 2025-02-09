use std::cmp::{max, min};

fn main() {
    let mut s = vec![0; 300001];
    for k in 1..=55 {
        s[k] = (100003 - 200003 * k as i64 + 300007 * (k as i64).pow(3)) % 1_000_000;
    }
    for k in 56..=300000 {
        s[k] = (s[k - 24] + s[k - 55]) % 1_000_000;
    }

    let mut cuboids = vec![];
    for n in 1..=50000 {
        let x0 = s[6 * n - 5] % 10000;
        let y0 = s[6 * n - 4] % 10000;
        let z0 = s[6 * n - 3] % 10000;
        let dx = 1 + (s[6 * n - 2] % 399);
        let dy = 1 + (s[6 * n - 1] % 399);
        let dz = 1 + (s[6 * n] % 399);
        cuboids.push(((x0, y0, z0), (dx, dy, dz)));
    }

    let combined_volume = calculate_combined_volume(&cuboids);
    println!("Combined volume: {}", combined_volume);
}

fn calculate_combined_volume(cuboids: &Vec<((i64, i64, i64), (i64, i64, i64))>) -> i64 {
    let mut volume = 0;

    for (i, &cuboid_a) in cuboids.iter().enumerate() {
        volume += cuboid_volume(cuboid_a);

        for &cuboid_b in &cuboids[i+1..] {
            if let Some(intersection) = intersect(cuboid_a, cuboid_b) {
                volume -= cuboid_volume(intersection);
            }
        }
    }

    volume
}

fn cuboid_volume(cuboid: ((i64, i64, i64), (i64, i64, i64))) -> i64 {
    let ((x0, y0, z0), (dx, dy, dz)) = cuboid;
    dx * dy * dz
}

fn intersect(a: ((i64, i64, i64), (i64, i64, i64)), b: ((i64, i64, i64), (i64, i64, i64))) -> Option<((i64, i64, i64), (i64, i64, i64))> {
    let ((x0_a, y0_a, z0_a), (dx_a, dy_a, dz_a)) = a;
    let ((x0_b, y0_b, z0_b), (dx_b, dy_b, dz_b)) = b;

    let x1_a = x0_a + dx_a;
    let y1_a = y0_a + dy_a;
    let z1_a = z0_a + dz_a;

    let x1_b = x0_b + dx_b;
    let y1_b = y0_b + dy_b;
    let z1_b = z0_b + dz_b;

    let x0 = max(x0_a, x0_b);
    let y0 = max(y0_a, y0_b);
    let z0 = max(z0_a, z0_b);

    let x1 = min(x1_a, x1_b);
    let y1 = min(y1_a, y1_b);
    let z1 = min(z1_a, z1_b);

    if x0 <= x1 && y0 <= y1 && z0 <= z1 {
        let dx = x1 - x0;
        let dy = y1 - y0;
        let dz = z1 - z0;
        Some(((x0, y0, z0), (dx, dy, dz)))
    } else {
        None
    }
}

