use std::{fs::read_to_string, hash::Hash};
use chrono::{Local,Utc};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    println!("{}",is_even(5));
    println!("{}",fib(6));
    let my_string = String::from("Hii there");
    let length = find_string_length(&my_string);
    println!("String length is :{}",length);
    let user1 = User{
        first_name:String::from("Rohan"),
        last_name:String::from("Raj"),
        age:53
    };
    println!("{}",user1.first_name);
    println!("{}",user1.last_name);
    println!("{}",user1.age);
    let rect1 = Rect{
        width:32,
        height:16
    };
    println!("{}",rect1.area());
    println!("{}",rect1.perimeter());
    let my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction);
    let index = find_first_a(String::from("Harkirat"));
    // if index == -1 {
    //     println!("a not found!");
    // }else {
    //     println!("Index is found at {}",index);
    // }
    match index {
        Some(value)=>println!("Index is found at {}",value),
        None => println!("Index is not found!")
    }
    let result = read_to_string("a.txt");
    match result {
        Ok(data)=>println!("Data is {}",data),
        Err(err)=>println!("Error")
    }
    let now = Utc::now();
    println!("{}",now);
    let formatted = now.format("%d %m %Y");
    println!("{}",formatted);
    let local = Local::now();
    println!("{}",local);
    // mutability
    let mut name = String::from("Harkirat");
    name.push_str(" Singh");
    println!("{}",name);
    // Vectors
    let mut vec = Vec::new();
    vec.push(2);
    vec.push(3);
    println!("{:?}",vec);
    println!("{:?}",even_filter(vec));
    // HashMaps
    let mut users = HashMap::new();
    users.insert(String::from("Harkirat"),32);
    users.insert(String::from("Raman"),38);
    let first_user_age = users.get("wow");
    let second_user_age = users.get("Raman");
    match first_user_age {
        Some(value) => println!("{}",value),
        None => println!("User not found!")
    }
    match second_user_age {
        Some(value) => println!("{}",value),
        None => println!("User not found!")
    }
    let vect = vec![(String::from("Harkirat"),32),(String::from("Raman"),37)];
    let hsm = group_values_by_keys(vect);
    println!("{:?}",hsm);

    // using traits
    let user = User2 {
        name:String::from("Harkirat"),
        age:23
    };
    println!("{}",user.summarize());
}

fn is_even(num :i32)->bool{
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num :i32)->i32{
    if num == 0 || num == 1 {
        return num;
    }
    return fib(num-1)+fib(num-2);  // 0 1 1 2 3 5
}

fn find_string_length(s:&str)->usize{
    return s.chars().count();
}

struct User {
    first_name:String,
    last_name:String,
    age:i32
}

struct Rect {
    width:i32,
    height:i32
}

impl Rect {
    fn area(&self)->i32{
        return self.width * self.height;
    }
    fn perimeter(&self)->i32{
        return 2*(self.width + self.height);
    }
}

enum Direction {
    North,
    East,
    West,
    South
}

fn move_around(direction:Direction){

}

fn find_first_a(s: String)->Option<i32>{
    for(index,char) in s.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn even_filter(vec:Vec<i32>)->Vec<i32>{
    let mut result = Vec::new();
    for el in vec {
        if el % 2 == 0 {
            result.push(el);
        }
    }
    return result;
}

fn group_values_by_keys(vec:Vec<(String,i32)>)->HashMap<String,i32>{
    let mut hm = HashMap::new();
    for (key,value) in vec {
        hm.insert(key,value);
    }
    return hm;
}

trait Summary {
    fn summarize(&self)->String;
}

struct User2 {
    name:String,
    age:u32
}

impl Summary for User2 {
    fn summarize(&self)->String {
        return format!("The name is {} and age is {}",self.name,self.age);
    }
}