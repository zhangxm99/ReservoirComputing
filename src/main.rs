mod network;
mod reservoir;
mod param;

fn main() {
    let mut lsm = network::Network::new();
    let pattern = [0, 1, 1,0,0,1,0,0,1,0,1,0,1,0,0,0,0,1,0,1,0,0,0,0,0,1,1,0,0,0,0,0,0,1,0,1,1,1,0,1];
    let repeat_pattern = std::iter::repeat_with(|| pattern.iter()).flatten();

    let mut pred = 0;
    let mut mis = 0;
    let mut sum = 0;
    for (ith,&i) in repeat_pattern.enumerate(){
        sum += 1;
        if pred != i{
            mis += 1;
            lsm.train(pred,i);
        }
        lsm.clean_spike();
        pred = lsm.input(i);
        println!("got{},pred{}",i,pred);
        // if ith % 100 == 0{println!("{}",mis as f32/sum as f32);}

    }
}
