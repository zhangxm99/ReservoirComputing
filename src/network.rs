use crate::reservoir::Reservoir;

pub struct Network{
    r: Reservoir::<30,30>,
    choosen :Vec<(usize,usize)>,
    w:Vec<f32>,
    l:usize
}
impl Network {
    pub fn new() -> Self{
        // let choosen = vec![(0,1),(0,3),(0,2),(0,15),(0,19),(3,13),(3,1),(6,14),(7,24),(11,11),(11,2),(13,2),(18,3),(24,7),(24,9),(28,15),(28,20)];
        let mut choosen = Vec::<(usize,usize)>::new();
        for i in 0..30{
            for j in 0..30{
                choosen.push((i,j));
            }
        }
        let l = choosen.len();
        Self{
            r:Reservoir::new(),
            choosen,
            w: vec![0.0;l+1],
            l
        }
    }
    //编码层，将输入编码为多神经元脉冲送入库中，这里送入是随机选的
    fn encode(input:i32) -> Vec::<((usize,usize),i32)>{
        let mut v:Vec<((usize,usize),i32)> = Vec::new();
        for i in 0..30{
            for j in 0..30{
                v.push(((i as usize,j as usize),0));
            }
        }
        match input{
            // 0 => vec![((1,7),0),((3,9),0),((4,11),0),((9,7),0),((14,23),0),((19,11),0),((23,4),0),((27,26),0),((29,29),0),((14,9),0)],
            // 1 => vec![((1,7),1),((3,9),1),((4,11),1),((9,7),1),((14,23),1),((19,11),1),((23,4),1),((27,26),1),((29,29),1),((14,9),1)],
            // _ => vec![]
            0 => v,
            1 => v.iter().map(|(tu,num)| (*tu,1)).collect::<Vec<((usize,usize),i32)>>(),
            _ => vec![]
        }
    }
    //解码层，训练理解水库中的信息
    fn decode(&mut self) -> i32{
        let mut res = self.w[0];
        let mut i = 1;
        for (x,y) in self.choosen.clone(){
            let val = self.r.matrix[x][y].isspike as i32 as f32;
            res += val * self.w[i];
            i+=1;
        }
        // self.r.clean_spike();
        (res > 0.0) as i32
    }
    //接受输入，返回输出，是整个网络的对外接口
    pub fn input(&mut self,i:i32) -> i32{
        // match i{
        //     1 => {
        //         for i in 0..4{
        //             self.r.input_spike_train(Self::encode(1));
        //         }
        //     }
        //     0 => {
        //         for i in 0..2{
        //             self.r.input_spike_train(Self::encode(1));
        //         }
        //     }
        //     _ => ()
        // }
        // self.r.input_spike_train(Self::encode(i));
        match i{
            1 => {
                self.r.input_spike_train(Self::encode(1));
                self.r.input_spike_train(Self::encode(0));
                // self.r.input_spike_train(Self::encode(0));
            }
            0 =>{
                self.r.input_spike_train(Self::encode(0));
                self.r.input_spike_train(Self::encode(1));
                // self.r.input_spike_train(Self::encode(1));
            }
            _=>()
        }
        // for i in 0..30{
        //     for j in 0..30{
        //         print!("{} ",self.r.matrix[i][j].isspike as i32);
        //     }
        //     print!("\n");
        // }
        // println!("");
        Self::decode(self)
    }
    pub fn clean_spike(&mut self){
        self.r.clean_spike();
    }
    //训练读出网络
    pub fn train(&mut self,pred:i32,real:i32){
        if real == 1{
            self.w[0] += 1.0;
        } else{
            self.w[0] -= 1.0;
        }
        let mut i = 1;
        for (x,y) in self.choosen.clone(){
            let val = self.r.matrix[x][y].isspike as i32;
            if val == real{
                self.w[i] += 1.0;
            } else{
                self.w[i] -= 1.0;
            }
            i += 1;
        }
        // println!("{:?}",self.w);
    }
}