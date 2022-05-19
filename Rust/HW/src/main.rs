use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;
use chrono::Local;

const L:usize = 12;
const GSIZE:usize = 256;
const ALIVE:usize = 15;


fn get_unix_epoc() -> u64{
    let dt = Local::now();
    dt.timestamp() as u64
}


fn generate(gen:&mut Xoshiro256StarStar)->Vec<i32> {

    // vec![rand::thread_rng().gen_range(0..123); L]
    let mut res = Vec::with_capacity(12);
    for _i in 0..12 {
        let tmp = gen.gen_range(0..122);
        res.push(tmp);
    }
    res
}

fn distance(x:&Vec<i32>, y:&Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..L {
        res += (x[i] - y[i]).abs();
    }
    res
}

fn cross(x:&Vec<i32>, y:&Vec<i32>, gen:&mut Xoshiro256StarStar) -> Vec<i32> {
    let mut res = Vec::with_capacity(L);
    
    // let r = rand::thread_rng().gen_range(0..4096);
    let r = gen.gen_range(0..4096);
    for shift in 0..L {
        if r >> shift & 1 == 1 {
            res.push(x[shift]);
        }else{
            res.push(y[shift]);
        }
    }

    res
}


fn next_generation(g:&Vec<Vec<i32>>, ans:&Vec<i32>, gen:&mut Xoshiro256StarStar)->Vec<Vec<i32>> {
    let mut score:Vec<(&Vec<i32>, i32)>  = g.iter().map(|x| (x, distance(x, ans))).collect();
    score.sort_unstable_by(|x, y| x.1.cmp(&y.1));
    // score.into_iter().map(|x| x.0.to_vec()).collect()
    
    let mut mem = Vec::from_iter(score.into_iter().take(ALIVE).map(|x| x.0.to_vec()));
    

    for i in 0..ALIVE {
        let reject = gen.gen_range(0..4096);
        for j in i+1..ALIVE {
            if reject >> j &1 == 1{
                let tmp = cross(&mem[i],&mem[j], gen);
                mem.push(tmp);
            }
        }
    }
    

    let mut res = Vec::with_capacity(GSIZE);
    let mut r:usize = gen.gen_range(0..4294967296);
    let mut shift = 0;
    
    for i in mem.into_iter() {      /* 全体からドロップダウン */
        if r >> shift & 1 == 1 {
            res.push(i);
        }
        shift += 1;
        if shift == 32 {
            shift = 0;
            r = gen.gen_range(0..4294967296);
        }
    }

    // println!("{}", res.len());

    while res.len() < GSIZE {
        res.push(generate(gen));
    }

    res
}


fn main() {
    let mut rng = rand_xoshiro::Xoshiro256StarStar::seed_from_u64(get_unix_epoc());

    let mut generation:Vec<Vec<i32>> = (0..128).into_iter().map(|_x| generate(&mut rng)).collect();
    let ans = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]; // -> "Hello World!"
    let mut loop_cnt:i128 = 0;

    while distance(&generation[0], &ans) != 0 {
        if loop_cnt % 10000 == 0 {
            // println!("{:?}\n{:?}", generation[0], generation[3]);
            /*for i in generation.iter() {
                println!("{:?}", i);
            }*/
            let s:String = generation[0].iter().map(|&x| char::from_u32(x as u32).unwrap()).collect();
            println!("Str:{}\tScore:{}\tcnt:{}", s, distance(&generation[0], &ans), loop_cnt);
        }
        
        generation = next_generation(&generation, &ans, &mut rng);
        loop_cnt += 1;
    }

    let s:String = generation[0].iter().map(|&x| char::from_u32(x as u32).unwrap()).collect();
    println!("Str:{}\tScore:{}\tcnt:{}", s, distance(&generation[0], &ans), loop_cnt);
}
