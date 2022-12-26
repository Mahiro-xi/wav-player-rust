use std::io::Cursor;
pub fn getwav() -> Cursor<&'static [u8]> {
    // wavファイルの埋め込み
    let loadwav = include_bytes!("examples/music.wav");
    // loadwav (include_bytes!)を入力ストリームに変換
    Cursor::new(loadwav)
}
