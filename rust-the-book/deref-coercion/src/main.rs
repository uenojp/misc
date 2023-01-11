// 参照外し型強制
// ref. https://doc.rust-jp.rs/book-ja/ch15-02-deref.html
// T: Deref<Target=U>の時、&Tから&U
// T: DerefMut<Target=U>の時、&mut Tから&mut U
// T: Deref<Target=U>の時、&mut Tから&U

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// 普通&strにするが、今回は素直な型強制を見たいので&String（&strでも通る）
fn f(s: &String) {}

fn g(s: &mut String) {}

fn main() {
    //  T == MyBox<String>
    //  U == String == Deref of Box<String>

    // 1. T: Deref<Target=U>の時、&Tから&U
    // &Box<String> to &String
    let b = MyBox::new(String::from("hello"));
    f(&b);

    // 2. T: DerefMut<Target=U>の時、&mut Tから&mut U
    // &mut Box<String> to &mut String
    let mut b = MyBox::new(String::from("hello"));
    g(&mut b);

    // 3. T: Deref<Target=U>の時、&mut Tから&U
    // &mut Box<String> to &String
    let mut b = MyBox::new(String::from("hello"));
    f(&mut b);
}
