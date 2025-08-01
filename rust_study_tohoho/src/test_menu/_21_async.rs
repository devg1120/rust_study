#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
   println!("-------------------- async");
   use futures::executor::block_on;
   
   struct Song {
       lyric: String,
   }
   
   async fn learn_and_sing() {
       let song = learn_song().await;
       sing_song(song).await;
   }
   
   async fn learn_song() -> Song {
       let song = Song { lyric: String::from("La la la...") };
       println!("Learned song");
       return song;
   }
   
   async fn sing_song(song: Song) {
       println!("{}", song.lyric);
   }
   
   async fn dance() {
       println!("Dance");
   }
   
   async fn async_main() {
       let f1 = learn_and_sing();	// 歌を習って歌う
       let f2 = dance();			// ダンスする
       futures::join!(f1, f2);
   }

    block_on(async_main());

}
