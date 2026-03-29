pub struct CPU{
    pc: u32,
    reg: [u32; 32],
    program: Vec<u32>,
}

impl CPU{
    fn new() -> Self {
        CPU {
            pc: 0,
            reg: [0; 32],       
            program: Vec::new(),
        }
     }

    fn fetch(){
        
    }

    fn decode(){

    }

    fn execute(){

    }
}