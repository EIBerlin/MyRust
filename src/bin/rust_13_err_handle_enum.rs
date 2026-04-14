enum Result<K, E> {
    Ok(K),
    Err(E),
}

impl<K: std::fmt::Display, E: std::fmt::Display> Result<K, E> {
    fn get(&self) {
        match self {
            Result::Ok(k) => println!("{k}"),
            Result::Err(e) => println!("{e}"),
        }
    }
}

fn test(v: u8) {
    if v == 1 {
        Result::<&str, &str>::Ok("IAMOK").get();
    } else {
        Result::<&str, &str>::Err("IAMERROR").get();
    }
}

pub fn main() {
    test(1); // Ok
    test(2); // Err
}

// ဒီ Code နှစ်ခုရဲ့ ကွာခြားချက်က Rust ရဲ့ **Generics** နဲ့ **Traits (Display)** ကို ကိုင်တွယ်ပုံမှာ ရှိပါတယ်။ ပထမ code မှာ ဘာကြောင့် Error တက်ပြီး ဒုတိယ code မှာ ဘာကြောင့် အဆင်ပြေသွားလဲဆိုတာ အသေးစိတ် ရှင်းပြပေးပါ့မယ်။

// ---

// ### ၁။ `std::fmt::Display` ဆိုတဲ့ Trait Bound လိုအပ်ချက်

// ပထမ code မှာ Error တက်ရတဲ့ အဓိက အကြောင်းရင်းက `println!("{k}")` ကြောင့်ပါ။

// * Rust မှာ `{}` (Display formatter) ကို သုံးပြီး တစ်ခုခုကို print ထုတ်ချင်ရင် အဲဒီ data type က **`std::fmt::Display`** ဆိုတဲ့ trait ကို implement လုပ်ထားဖို့ လိုအပ်ပါတယ်။
// * သင် သတ်မှတ်ထားတဲ့ `Result<K, E>` မှာ `K` နဲ့ `E` က ဘာ type ဖြစ်မလဲဆိုတာ Rust က ကြိုမသိပါဘူး။ အကယ်၍ တစ်ယောက်ယောက်က Display trait မရှိတဲ့ (print ထုတ်လို့မရတဲ့) structure တစ်ခုကို `K` နေရာမှာ ထည့်သုံးလိုက်ရင် `println!` က အလုပ်လုပ်မှာ မဟုတ်ပါဘူး။

// **Fixed Code ထဲက အဖြေ:**
// ```rust
// impl<K: std::fmt::Display, E: std::fmt::Display> Result<K, E> { ... }
// ```
// ဒီနေရာမှာ `K: std::fmt::Display` လို့ ရေးလိုက်တာဟာ **"ဒီ `get()` function ကို `K` နဲ့ `E` တို့က print ထုတ်လို့ရတဲ့ (Display trait ရှိတဲ့) type တွေဖြစ်မှသာ သုံးခွင့်ပြုမယ်"** လို့ ကန့်သတ်ချက် (Bound) ထည့်လိုက်တာပါ။ ဒါကြောင့် Compiler က စိတ်ချလက်ချ print ထုတ်ခွင့် ပေးလိုက်တာ ဖြစ်ပါတယ်။

// ---

// ### ၂။ Turbofish Operator `::<>` ကို အသုံးပြုခြင်း

// ဒုတိယ code ရဲ့ `test` function ထဲမှာ `Result::<&str, &str>::Ok(...)` လို့ ပြင်လိုက်တာကို တွေ့ရမှာပါ။ ဒါကို Rust မှာ **Turbofish Operator** လို့ ခေါ်ပါတယ်။

// **ဘာကြောင့် သုံးရတာလဲ?**
// Rust compiler က တခါတလေ generic type တွေကို သူ့အလိုအလျောက် ခန့်မှန်း (Inference) ရတာ ခက်ခဲတတ်ပါတယ်။ အထူးသဖြင့် logic က ခွဲထွက်သွားတဲ့အခါမျိုးမှာပါ။
// * `Result::Ok("IAMOK")` လို့ပဲ ရေးရင် Compiler က "အိုကေ... `K` ကတော့ `&str` ပေါ့၊ ဒါပေမဲ့ `E` (Error type) က ဘာဖြစ်မလဲ?" ဆိုတာ ဝေခွဲမရဖြစ်တတ်ပါတယ်။
// * `Result::<&str, &str>::Ok(...)` လို့ ရေးပေးခြင်းဖြင့် `K` ကလည်း String slice ပဲ၊ `E` ကလည်း String slice ပဲလို့ တိတိကျကျ ပြောပြလိုက်တာ ဖြစ်ပါတယ်။



// ---

// ### ၃။ Logic စီးဆင်းပုံ အနှစ်ချုပ်

// ၁။ **Generic Definition:** `Result<K, E>` ကို ပုံစံတူ data နှစ်ခု သိမ်းနိုင်အောင် တည်ဆောက်ထားတယ်။
// ၂။ **Constraint:** `impl` မှာ `Display` bound ထည့်ထားလို့ `println!` က စိတ်ချရသွားတယ်။
// ၃။ **Concrete Types:** `main -> test` ထဲမှာ `&str` (String slice) တွေကို data အနေနဲ့ ထည့်သုံးထားတယ်။ `&str` ဟာ Display trait ရှိပြီးသားမို့လို့ အဆင်ပြေပြေ print ထွက်လာတာပါ။

// ### Output:
// ```text
// IAMOK
// IAMERROR
// ```

// ဒီပြင်ဆင်ချက်က Rust ရဲ့ **Type Safety** ကို အပြည့်အဝ အသုံးချသွားတာ ဖြစ်ပါတယ်။ အခု Code က ပိုပြီး စနစ်ကျသွားပါပြီ။
