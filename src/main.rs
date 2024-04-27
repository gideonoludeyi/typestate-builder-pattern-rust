struct NoA;
struct MyA;

struct NoB;
struct MyB;

struct NoC;
struct MyC;

struct ThreePieceBuilder<A, B, C> {
    a: A,
    b: B,
    c: C,
}

impl ThreePieceBuilder<NoA, NoB, NoC> {
    fn new() -> Self {
        Self {
            a: NoA,
            b: NoB,
            c: NoC,
        }
    }
}

impl<B, C> ThreePieceBuilder<NoA, B, C> {
    fn with_a(self, a: MyA) -> ThreePieceBuilder<MyA, B, C> {
        ThreePieceBuilder {
            a,
            b: self.b,
            c: self.c,
        }
    }
}

impl<A, C> ThreePieceBuilder<A, NoB, C> {
    fn with_b(self, b: MyB) -> ThreePieceBuilder<A, MyB, C> {
        ThreePieceBuilder {
            a: self.a,
            b,
            c: self.c,
        }
    }
}

impl<A, B> ThreePieceBuilder<A, B, NoC> {
    fn with_c(self, c: MyC) -> ThreePieceBuilder<A, B, MyC> {
        ThreePieceBuilder {
            a: self.a,
            b: self.b,
            c,
        }
    }
}

impl ThreePieceBuilder<MyA, MyB, MyC> {
    fn build(self) -> String {
        "Done".into()
    }
}

fn main() {
    let s = ThreePieceBuilder::new()
        .with_c(MyC)
        .with_b(MyB)
        .with_a(MyA)
        .build();
    println!("{s:#?}");
}
