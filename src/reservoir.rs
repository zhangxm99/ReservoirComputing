use rand::Rng;

#[derive(Debug)]
pub struct Neuron{
    //膜电势
    pub v:f32,
    //记录是否此次脉冲了
    pub isspike: bool,
    //阈值
    threshold:f32,
    //输入权重,正是加强，负是抑制
    w: [[f32;30];30],
    //输出连接神经元坐标
    nexts: Vec<(usize,usize)>
}
impl Neuron{
    fn new() -> Neuron{
        let mut rng = rand::thread_rng();
        let threshold = rng.gen_range(1.0..18.0);

        Self{
            v:0.0,
            isspike:false,
            threshold,
            w:[[1.0;30];30],
            nexts: Vec::new()
        }
    }
    fn spike_from(&mut self,from:(usize,usize)) -> u8{
        let (x,y) = from;
        self.v += self.w[x][y];
        if self.v > self.threshold{
            self.isspike = true;
            self.v = 0.2;
            1
        } else{
            0
        }
        // else{
        //     self.isspike = false;
        //     //自定衰减系数超参数
        //     self.v = self.v as f32*0.8;
        //     0
        // }
    }
    fn leak(&mut self){
        self.v *= 0.8;
    }
}


#[derive(Debug)]
pub struct Reservoir<const H:usize,const W:usize>{
    pub matrix:Vec<Vec<Neuron>>
}

impl<const H:usize,const W:usize> Reservoir<H,W>{
    //随机生成连接
    pub fn new() -> Self{
        let mut rng = rand::thread_rng();
        
        let mut matrix:Vec<Vec<Neuron>> = Vec::with_capacity(H);
        for _ in 0..H{
            let mut row:Vec<Neuron> = Vec::with_capacity(W);
            for _ in 0..W{
                row.push(Neuron::new());
            }
            matrix.push(row);
        }
        for i1 in 0..H{
            for j1 in 0..W{
                for i2 in 0..H{
                    for j2 in 0..W{

                        if i1 == i2 && j1 == j2 {continue;}
                        //欧拉距越近两者间有连接的概率越大，这里允许了单层之间神经元的连接，可以试试如果只允许跨层连接会怎么样
                        let x_diff = i1 as i32 - i2 as i32;
                        let y_diff = j1 as i32 - j2 as i32;
                        let distance2 = (x_diff * x_diff + y_diff * y_diff) as i32;
                        let p = 0.4*f64::exp(-distance2 as f64/4.0f64);
                        // println!("{}",p);
                        if rng.gen_range(0.0..1.0) <= p {
                            matrix[i1][j1].nexts.push((i2,j2));
                            matrix[i2][j2].w[i1][i2] = rng.gen_range(-1.0..4.0);
                        }

                    }
                }
            }
        }
        Self{matrix}
    }

    fn propagation(&mut self,to: (usize,usize),from: (usize,usize)){
        let (x,y) = to;
        let out = self.matrix[x][y].spike_from((x,y));
        // println!("{:?}",from);
        if out == 1{
            for nxt in self.matrix[x][y].nexts.clone(){
                self.propagation(nxt,to);
            }
        } 
    }
    fn leak(&mut self){
        for i in 0..H{
            for j in 0..W{
                self.matrix[i][j].leak();
            }
        }
    }

    pub fn input_spike_train(&mut self,input: Vec::<((usize,usize),i32)>){
        for ((x,y),val) in input{
            if val == 1{
                self.propagation((x,y),(x,y));
            } 
        }
        self.leak();
    }
    pub fn clean_spike(&mut self){
        for i in 0..H{
            for j in 0..W{
                self.matrix[i][j].isspike = false;
            }
        }
    }
}