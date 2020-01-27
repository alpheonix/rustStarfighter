mod spaceship; 
use spaceship::create_spaceship_template;
use std::collections::HashMap;
use std::any::Any;
use serde_json::Map;
use serde_json::Number;
extern crate rand;
use rand::Rng;

enum Value {
    Null,
    Bool(bool),
    i64,
    String,
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://mysterious-fortress-67328.herokuapp.com/spaceship/getAll")
        .await?
        .json::<HashMap<String,serde_json::Value>>()
        .await?;
    let mut rng = rand::thread_rng();
    let rand1 = rng.gen_range(0, 31);
    let rand2 = rng.gen_range(0, 31);
    
   println!("Hello Benj!");
    let mut x_wing = spaceship::Spaceship{_id:resp["spaceships"][rand1]["_id"].as_str().unwrap().to_string(),
                                            name:resp["spaceships"][rand1]["name"].as_str().unwrap().to_string(),
                                            speed: resp["spaceships"][rand1]["speed"].as_i64().unwrap(),
                                            hp: resp["spaceships"][rand1]["hp"].as_i64().unwrap(),
                                            damage: resp["spaceships"][rand1]["damage"].as_i64().unwrap(),
                                            adventage: false};
    println!("{}",x_wing.name);
   let mut executor = spaceship::Spaceship{_id:resp["spaceships"][rand2]["_id"].as_str().unwrap().to_string(),
                                            name:resp["spaceships"][rand2]["name"].as_str().unwrap().to_string(),
                                            speed: resp["spaceships"][rand2]["speed"].as_i64().unwrap(),
                                            hp: resp["spaceships"][rand2]["hp"].as_i64().unwrap(),
                                            damage: resp["spaceships"][rand2]["damage"].as_i64().unwrap(),
                                            adventage: false};
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
    println!("{}",executor.name);
    println!("{}",executor.hp);
    println!("{}",x_wing.name);
    println!("{}",x_wing.hp);

    Ok(())

    
}