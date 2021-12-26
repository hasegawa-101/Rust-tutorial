fn main() {
    // csvデータ
    let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record // テキスト行で処理を開始
            .split(',') // レコードをフィールドに分割
            .map(|field| field.trim()) // 各フィールドの空白を除去
            .collect(); // フォールドのコレクションを作る

        // デバッグ時のコード
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields); // 標準エラーに出力
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length); // 結果の出力
        }
    }
}
