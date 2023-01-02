use std::mem::MaybeUninit;
use std::sync::{mpsc, Mutex, Once};
use std::thread;
use crate::markdown::article::Article;
use crate::markdown::file_manager::{get_articles, get_articles_vector};

pub struct ArticleStatic {
    pub inner: Mutex<Vec<Article>>
}

pub fn articles() -> &'static ArticleStatic {
    static mut SINGLETON: MaybeUninit<ArticleStatic> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let articles = ArticleStatic{
                inner: Mutex::new(get_articles())
            };
            SINGLETON.write(articles);
        });
        SINGLETON.assume_init_ref()
    }
}

pub fn update_pool() {
    thread::spawn(|| {
        let a = articles();
        let mut data =  a.inner.lock().unwrap();
        *data = get_articles();
    });
    println!("Updated pool");
}