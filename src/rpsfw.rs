use std::thread;
use std::sync::{Arc, Mutex};

fn main(){
    let n_th = 2;
    let hands = Arc::new(Mutex::new(vec![0; n_th]));
    let mut threads = Vec::new();
    for i in 0..n_th {
        let hands = hands.clone();
        threads.push(thread::spawn(move || {
            let mut hands = hands.lock().unwrap();
            let hand = rand::random::<u8>() % 5;
            hands[i] = hand;
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let hands = hands.lock().unwrap();
    let hand1 = hands[0];
    let hand2 = hands[1];
    if hand1 == hand2 {
        println!("Tie!");
    } else if (hand1 == 0 && hand2 == 2) ||
              (hand1 == 0 && hand2 == 4) ||
              (hand1 == 1 && hand2 == 0) ||
              (hand1 == 1 && hand2 == 4) ||
              (hand1 == 2 && hand2 == 1) ||
              (hand1 == 2 && hand2 == 4) ||
              (hand1 == 3 && hand2 == 0) ||
              (hand1 == 3 && hand2 == 1) ||
              (hand1 == 3 && hand2 == 2) ||
              (hand1 == 4 && hand2 == 3) {
        println!("Player 1 wins!");
    } else {
        println!("Player 2 wins!");
    }
}
