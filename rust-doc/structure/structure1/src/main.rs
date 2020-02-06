
// 構造体の定義
struct User { 
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn show(user: &User){
    println!("=======================");
    println!("     username: {}", user.username);
    println!("        email: {}", user.email);
    println!("sign_in_count: {}", user.sign_in_count);
    println!("       active: {}", user.active);
    println!("=======================");
}

fn main () {
    // 構造体のインスタンス生成
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    show(&user1);

    // メンバーの値を変更
    let user2 = build_user(String::from("test@example"), String::from("test_user"));

    show(&user2);

    // 別インスタンスからインスタンスを生成
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    show(&user3);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
