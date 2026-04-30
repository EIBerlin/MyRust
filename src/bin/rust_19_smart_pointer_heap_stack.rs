fn main() {
    /* Box::new(5) က နံပါတ် 5 ကို Heap ပေါ်မှာ သိမ်းပေးတယ်။
    b က အဲ့ဒီ Heap က data ကို ညွှန်ပြနေတဲ့ ပိုင်ရှင် (Owner) ဖြစ်တယ်။
    b ရဲ့ scope ကုန်ရင်၊ b ပါ၊ သူပိုင်တဲ့ Heap data ပါ အလိုအလျောက် မှတ်ဉာဏ်ကနေ ပျက်သွားတယ်။ */
    let b = Box::new(5);

    println!("b' value = {}", b);

    println!("b' pointer address (of Stack) = {:p}", &b);

    println!("5' heap address = {:p}", b.as_ref()); // usage 1
    println!("5' heap address = {:p}", &*b); // usage 2
}
