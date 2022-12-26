use rodio::{source::Source, Decoder, OutputStream};
mod getlength;
fn main() {
    // 出力デバイスの決定
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // wavファイルの読み込み処理
    let cursor = getlength::binwav::getwav();
    let source = Decoder::new(cursor).unwrap();
    // 再生処理
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // プロセス継続処理
    // getlengthからwavファイルの長さ(sec)取得
    let mainlength = getlength::getlength();
    println!("{} sec", mainlength);
    // wavの再生終了までsleepする。
    std::thread::sleep(std::time::Duration::from_secs(mainlength.into()));
}
