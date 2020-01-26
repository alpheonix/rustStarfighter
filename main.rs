mod spaceship; 
use spaceship::create_spaceship_template;
fn main() {
    println!("Hello Benj!");
    let mut x_wing = create_spaceship_template();
    println!("{}",x_wing.name);
    let mut executor = spaceship::Spaceship{_id:"2".to_string(),name:"Executor".to_string(),speed: 4,hp: 600000,damage: 100000,adventage: false};
    println!("{}",executor.name);
    println!("PIOU PIOU PIOU");
    let coef:i64;
    if(x_wing.speed<executor.speed){
        executor.adventage = true;
        coef = executor.speed/x_wing.speed;
    }else{
        x_wing.adventage = true;
        coef = x_wing.speed/executor.speed;
    }
    let mut temp = coef;
    while(x_wing.hp>0 && executor.hp>0){
        if(temp>=1){
            if(x_wing.adventage){
                executor.hp -= x_wing.damage;
                temp -= 1;
            }else{
                x_wing.hp -= executor.damage;
                temp -= 1;
            }

        }else{
            
            if(x_wing.adventage){
                x_wing.hp -= executor.damage;
                temp +=coef;
            }else{
                executor.hp -= x_wing.damage;
                temp +=coef ;
            }
        }
    }
    println!("{}",executor.hp);
    println!("{}",x_wing.hp);

    // call api et miss 
}