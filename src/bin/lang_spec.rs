
#[cfg(test)]
mod lang_spec {

    #[test]
    fn test001() {
        struct Point {
            x:i32,
            y:i32
        }

        let a = Point{x:10, y:100};

        assert_eq!(a.x, 10);
        assert_eq!(a.y, 100);

        // 所有権（Ownership）の移動
        fn func(p: Point) -> i32 {
            p.x + p.y
        }

        assert_eq!(func(a), 110);
    }

    #[test]
    fn test002() {
        //借用（borrowing）
        fn func(x: &i32) -> i32 {
            x * 2
        }

        let x = 100;
        assert_eq!(func(&x), 200);
    }

    #[test]
    fn test003() {
        //借用（mutable borrowing）
        struct Point {x:i32,y:i32}

        // mutableじゃないといけない
        let mut a = Point{x:10, y:1};
        
        fn func(p: &mut Point) -> i32 {
            p.x * p.y
        }

        //mutable 参照を渡す
        assert_eq!(func(&mut a), 10);
    }

    #[test]
    fn test004() {
        // tuple
        let (a, b) = (1, 2);

        assert_eq!(a, 1);
        assert_eq!(b, 2);

        // 関数の戻り値にTupleを利用する
        fn func() -> (i32, i32) {
            (10, 20)
        }

        let (x, y) = func();
        assert_eq!(x, 10);
        assert_eq!(y, 20);

        // 構造体にtupleを利用できる
        struct A {
            x: (i32, i32)
        }

        let aa = A{x:(100, 200)};
    }

    #[test]
    fn test005() {
        // fieldをmutableにはできない
        struct A {
            x:i32
        }

        let a = A{x:100};
        // アップデート構文
        let a2 = A{..a};

        assert_eq!(a2.x, 100);

        struct Tuple(i32,i32);
        let ts = Tuple(100, 200);
        
        // 値の取り出し
        let Tuple(x, y) = ts;

        assert_eq!(x, 100);
        assert_eq!(y, 200);
    }

    #[test]
    fn test006() {
        // 直和型
        enum Message {
            A,
            B(i32),
            C{x:i32, y:i32},
        }

        let a = Message::A;
        let b = Message::B(100);
        let c = Message::C{x:100,y:100};

        // パターンマッチ
        let x = match a {
            Message::A => 100,
            Message::B(x) => x,
            Message::C{x:x, y:y} => x,
        };
    }
}