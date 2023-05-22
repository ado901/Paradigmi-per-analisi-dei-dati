

/* 
    Write a Rust function
        Given a list of positive integers values...
        How many times does the maximum change?
    Like counting the skyscrapers' tops
        … which can be seen, looking from left to right
    Implement it using fold and the functional paradigm
 */
fn main() {
    /* let arr: Vec<i32>=vec![2,5,10,1,3,15];
    print!("{}",count_changes(arr)); */
    /* let book: Vec<PhoneBook>=vec![PhoneBook{name:"John".to_string(), phone_number: 123456789}, PhoneBook{name:"Mary".to_string(), phone_number: 987654321}];
    println!("{}",get_phone_number("John".to_string(), book).unwrap_err()); */
    let mut ball: Ball = Ball{x: 0, y: 0, dx: 1, dy: 1};
    println!("{},{}",ball.x,ball.y);
    move_ball1(&mut ball);
    println!("{},{}",ball.x,ball.y);
    let mut ballnew: Ball=move_ball2(ball);
    println!("{},{}",ballnew.x,ballnew.y);

}
fn count_changes(v:Vec<i32>) -> i32 {
    v.iter().fold((0,0),|(max, count), &x: &i32| {
        if x > max {
            (x, count + 1)
        } else {
            (max, count)
        }
    }).1
}


/* Implement a f. for searching in a phone book: [(String, i32)]
get_phone_number(name: String, phone_book: [(String, i32)]) -> Result<i32, String>
Result if name not found:

    Define signature to return Result<i32, String>

Any implementation */
struct PhoneBook {
    name: String,
    phone_number: i32,
}

fn get_phone_number(name: String, phone_book: Vec <PhoneBook>) -> Result<i32, String>{
    for i in phone_book {
        if i.name == name {
            return Ok(i.phone_number);
        }
    }
    Err("Name not found".to_string())
}


   /*  Move a ball in Rust
        Advance a step, bounce at borders
    Define a Ball struct, with apprpriate fields
    Define a move_ball function
        Borrow the struct
    In the main
        Create an instance, call the function, then print fields
 */

struct Ball {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn move_ball1(b: &mut Ball) {
    if b.x+b.dx>100||b.x+b.dx<0{
        b.dx = -b.dx;
    }
    if b.y+b.dy>100||b.y+b.dy<0{
        b.dy = -b.dy;
    }
    b.x += b.dx;
    b.y += b.dy;
}


fn move_ball2(b: Ball)->Ball{
    let mut bnew = b;
    if bnew.x+bnew.dx>100  ||bnew.x+bnew.dx<0{
        bnew.dx = -bnew.dx;
    }
    if bnew.y+bnew.dy>100 || bnew.y+bnew.dy<0{
        bnew.dy = -bnew.dy;
    }
    bnew.x += bnew.dx;
    bnew.y += bnew.dy;   
    bnew
}