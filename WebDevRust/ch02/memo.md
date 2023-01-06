# 第2章

### 関数

```
fn void(){
    
}
fn return_void(){
    return ();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    println!("Hello, world!");

    let a = void();
    println!("{:?}", a);

    
    let a = return_void();
    println!("{:?}", a);
    
    println!("{}", add(123, 234))
}
```

### enum

coreで以下のように定義されている。
```
enum Result<T,E> {
    Ok(T),
    Err(E),
}
```

そのため、文字列を使う場合には以下のように記載する。
```
fn main() {
    let some: Result<&str, &str> = Ok("ok");
    println!("{:?}", some);
    
    let err: Result<&str, &str> = Err("err");
    println!("{:?}", err);

}
```

### unwrap/expect

Ok の場合は、問題なし。
```
fn main(){
    let input: Result<&str, &str> = Ok("ok");
    let input = input.unwrap();
    println!("{:?}", input);
}
```

Err (E)/Noneの場合には、panic になる。
```
fn main(){
    let input: Result<&str, &str> = Err("err");
    let input = input.unwrap();
    println!("{:?}", input);
}

```

### コメント

コメント内にテストコードを記載して cargo test を実行しても動作しないのは、バイナリクレートだから？
ライブラリクレートのときは、問題なく実行された。    