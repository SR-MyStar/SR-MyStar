fn main() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let mut x = x;
    x += 3;

    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!";
}
