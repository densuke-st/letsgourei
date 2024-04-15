// 出席番号を2つ表示するプログラム
// 1〜28番のうち、16と19を除外した番号のうち2つの番号をランダムに出力する。
// 最初に出力する番号の前には「メイン: 」と付けた上で出席番号を出す
// 次に出力する番号の前には「サブ: 」と付けた上で出席番号を出す
use rand::Rng;

fn main() {
    // 出席番号のリストを作る、16と19を除外する
    let mut numbers = vec![];
    for i in 1..29 {
        if i != 16 && i != 19 {
            numbers.push(i);
        }
    }

    // 乱数を生成する
    let mut rng = rand::thread_rng();

    // メインの出席番号を出力する
    let main = rng.gen_range(0..numbers.len());
    println!("メイン: {}", numbers[main]);

    // サブの出席番号を出力する
    numbers.remove(main);
    let sub = rng.gen_range(0..numbers.len());
    println!("サブ: {}", numbers[sub]);

}
