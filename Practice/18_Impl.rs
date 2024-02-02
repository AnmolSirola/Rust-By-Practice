
struct Temperature{
    degrees_f: f64,
}

fn show_temp(temp: Temperature){
    println!("{:?} degress in F", temp.degrees_f);
}

fn main(){
    let hot = Temperature{degrees_f: 99.9};
    show_temp(hot);
}
----------------------------------------------------------------

struct Temperature{
    degrees_f: f64,
}

impl Temperature{
    
    fn show_temp(temp: Temperature){
        println!("{:?} degress in F", temp.degrees_f);
    }

}

fn main(){
    let hot = Temperature{degrees_f: 99.9};
    Temperature::show_temp(hot);
}

-----------------------------------------------------------------
// self - instance of type(in our case we can say Temperature)

struct Temperature{
    degrees_f: f64,
}

impl Temperature{
    
    fn show_temp(&self){    // fn show_temp(temp: Temperature){
        println!("{:?} degress in F", self.degrees_f);
    }

}

fn main(){
    let hot = Temperature{degrees_f: 99.9};
    //Temperature::show_temp(hot);
    hot.show_temp();
}

------------------------------------------------------------------

struct Temperature{
    degrees_f: f64,
}

impl Temperature{
    
    fn show_temp(&self){    // fn show_temp(temp: Temperature){
        println!("{:?} degress in F", self.degrees_f);
    }

    fn freezing(d_f: f64) -> Temperature{
        Temperature{
            degrees_f: d_f
        }
    }

}

fn main(){
    let hot = Temperature{degrees_f: 99.9};
    //Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing(10.9);
    cold.show_temp();
}

----------------------------------------------------------------------------

struct Temperature{
    degrees_f: f64,
}

impl Temperature{
    
    fn show_temp(&self){    // fn show_temp(temp: Temperature){
        println!("{:?} degress in F", self.degrees_f);
    }

    fn freezing(d_f: f64) -> Self{ //Self means the type itself it has capital S
        Self{
            degrees_f: d_f
        }
    }

}

fn main(){
    let hot = Temperature{degrees_f: 99.9};
    //Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing(10.9);
    cold.show_temp();
}