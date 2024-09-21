pub struct Server{
    port: u16,
}

impl Server{
    pub fn init() -> Self{
        
        Self { port: 3000 }
    }
}