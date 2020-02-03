mod spaceship; 
extern crate rand;
use rand::Rng;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://mysterious-fortress-67328.herokuapp.com/spaceship/getAll")
        .await?
        .json::<serde_json::Value>()
        .await?;
    let mut rng = rand::thread_rng();
    let rand1 = rng.gen_range(0, 31);
    let rand2 = rng.gen_range(0, 31);
    
    let mut x_wing = spaceship::Spaceship{_id:resp["spaceships"][rand1]["_id"].as_str().unwrap().to_string(),
                                            name:resp["spaceships"][rand1]["name"].as_str().unwrap().to_string(),
                                            speed: resp["spaceships"][rand1]["speed"].as_i64().unwrap(),
                                            hp: resp["spaceships"][rand1]["hp"].as_i64().unwrap(),
                                            damage: resp["spaceships"][rand1]["damage"].as_i64().unwrap(),
                                            adventage: false,
                                            miss: 100};

   let mut executor = spaceship::Spaceship{_id:resp["spaceships"][rand2]["_id"].as_str().unwrap().to_string(),
                                            name:resp["spaceships"][rand2]["name"].as_str().unwrap().to_string(),
                                            speed: resp["spaceships"][rand2]["speed"].as_i64().unwrap(),
                                            hp: resp["spaceships"][rand2]["hp"].as_i64().unwrap(),
                                            damage: resp["spaceships"][rand2]["damage"].as_i64().unwrap(),
                                            adventage: false,
                                            miss: 100};
    println!("PIOU PIOU PIOU");
    let coef:i64;
    if x_wing.speed<executor.speed {
        executor.adventage = true;
        coef = executor.speed/x_wing.speed;
    }else{
        x_wing.adventage = true;
        coef = x_wing.speed/executor.speed;
    }

    match resp["spaceships"][rand1]["speed"].as_u64().unwrap() {
        0..=50 => x_wing.miss = 100,
        51..=100 => x_wing.miss = 80,
        101..=150 => x_wing.miss = 60,
        _ => println!("yo"),
    }

    match resp["spaceships"][rand2]["speed"].as_u64().unwrap() {
        0..=50 => executor.miss = 100,
        51..=100 => executor.miss = 80,
        101..=150 => executor.miss = 60,
        _ => println!("yo"),
    }

    println!("{ }",x_wing.miss);
    println!("{ }",executor.miss);
    let mut temp = coef;
    while x_wing.hp>0 && executor.hp>0 {
        
        if temp>=1 {
            if x_wing.adventage {
                if x_wing.miss > rng.gen_range(0, 100) {
                    executor.hp -= x_wing.damage;
                }
                temp -= 1;
                
            }else{
                if executor.miss > rng.gen_range(0, 100){
                x_wing.hp -= executor.damage;
                }
                temp -= 1;
            }

        }else{
            
            if x_wing.adventage{
                if executor.miss > rng.gen_range(0, 100){
                x_wing.hp -= executor.damage;
                }
                temp +=coef;
            }else{
                if x_wing.miss > rng.gen_range(0, 100){
                    executor.hp -= x_wing.damage;
                    
                }
                temp +=coef ;
            }
        }
    }
    println!("{}",executor.name);
    println!("{}",executor.hp);
    println!("{}",x_wing.name);
    println!("{}",x_wing.hp);

    Ok(())

    
}