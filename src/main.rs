// fn main() {
//     println!("Hello, world!");
//     let x= 100;
//     println!("x {}", x);

//     let is_family = true;
//     if is_family {
//         println!("is: {}", is_family )
//     }

//     let sentence = String::from("My name is Akshay");
//     let first_word = get_first_word(sentence);

//     println!("first word : {}", first_word);
//     let n = 1000;
//     for i in 0..n {
//         println!("Hello: {}", i)
//     }
// }

fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return  ans;
}

struct Rect {
    width: u32,
    height: u32
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
}

fn main() {
    let rect = Rect {
        width: 32,
        height: 50,
    };
    print!("The area of rectangle is {}", rect.area());
}
