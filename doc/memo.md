# blobオブジェクトの作成

## blobオブジェクトの元を作る

blobオブジェクトのフォーマットは`blog` [文字数]\0[内容]。`format`マクロで文字列を連結する。

```rust
// main.rs
fn main() {
    let blob = format!("blob {}\0{}", 11, "Hello World");

    println!("{}", blob);
    //=> blob 11Hello World
}
```

## ハッシュ化してオブジェクト化する

`sha1::{Sha1}`でハッシュ化する。

```rust
// main.rs

fn main() {
    let blob = format!("blob {}\0{}", 11, "Hello World");

    let blob_object = Sha1::digest(blob.as_bytes());

    println!("{:x}", blob_object);
    //=> 5e1c309dae7f45e0f39b1bf3ac3cd9db12e7d689
}
```

この`5e1c30`が正しいかを本家Gitで確認する。`Hello World`を`text.txt`に入力する（末尾に改行コードがあるとダメなことに注意）。ハッシュ化した値は`hash-object`コマンドで確認できる。

```bash
$ git hash-object text.txt
5e1c309dae7f45e0f39b1bf3ac3cd9db12e7d689
```

## ファイルを読み込む

`Hello World`が書き込まれた`text.text`を読み込みたい。

カレントディレクトリは`env::current_dir`で取得できる。ただし、Result型が返ってくるので、とりあえずmatchに渡して`println!`する。`Display`トレイトを実装していないと怒られるので、`.display()`をつけること。

```rust
// main.rs

fn main() {
    let abs_path = env::current_dir();

    match abs_path {
        Ok(path) => println!("{}", path.display()),
        //=> C:\github\vcrust
        Err(err) => println!("{}", err)
    };
}
```

パスが取得できていることを確認出来たら、`?`でエラー委譲しておく。

```rust
fn main() -> std::io::Result<()> {
    // ...

    let abs_path = env::current_dir()?;
}
```

`std::path::Path`はファイルやディレクトリのパス情報を扱うのに便利。これのいわばmutable版である`std::path::PathBuf`を使ってみる。

`push`を使用することでパスを追加していくことができる。

```rust
// main.rs

fn main() -> std::io::Result<()> {
    // ...

    let file = PathBuf::from("text.txt");

    println!("{}", file.display());

    Ok(())
}
```

[ファイルやディレクトリのパス文字列を構築／分割する (std::path::Path, PathBuf) - まくまく Rust ノート](https://maku77.github.io/p/36hr2bj/)

[How to Fix `T` doesn't implement std:fmt:Display in Rust? - Become A Better Programmer](https://www.becomebetterprogrammer.com/rust-fix-doesnt-implement-std-fmt-display/)
