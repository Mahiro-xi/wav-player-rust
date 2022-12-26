extern crate hound;
pub mod binwav;

pub fn getlength() -> u32 {
    // wavファイルを開く
    let cursor = binwav::getwav();
    // wavファイルをデコードする
    let reader = hound::WavReader::new(cursor).unwrap();
    // wavファイルのフォーマット情報を取得する
    let spec = reader.spec();
    // wavファイルのサンプル数を取得する
    let sample_count = reader.len() / 2;
    // wavファイルのサンプリング周波数を取得する
    let sampling_frequency = spec.sample_rate;
    // 長さを算出
    return sample_count / sampling_frequency;
}
