use core::time;
use std::{thread::sleep, time::Duration};
use indicatif;


pub fn progress_bar(){

    let pb = indicatif::ProgressBar::new(100);
    /*for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }*/
    for _ in 0..100 {
        do_hard_work();
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
fn do_hard_work(){
    sleep(time::Duration::from_millis(100));
}
