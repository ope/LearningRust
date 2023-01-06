# 第2章

### 関数

```
fn void(){
    
}
fn return_void(){
    return ();
}

fn main() {
    println!("Hello, world!");

    let a = void();
    println!("{:?}", a);

    
    let a = return_void();
    println!("{:?}", a);
}
```