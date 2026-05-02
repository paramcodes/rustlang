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