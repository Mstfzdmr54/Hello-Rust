fn main() {
    let mut data = String::from("Merhaba, Rust!");

    println!("Veri: {}", data);

    // Veriyi başka bir fonksiyona aktarıyoruz
    take_ownership(data);

    // 'data' artık geçerli değil ve kullanılamaz

    // Hata: 'data' geçerli değil
    // println!("Veri: {}", data);

    let num = 42;

    println!("Numara: {}", num);

    // 'num' değerini başka bir fonksiyona aktarıyoruz
    copy_value(num);

    // 'num' hala geçerli ve kullanılabilir
    println!("Numara: {}", num);
}

fn take_ownership(data: String) {
    // 'data' sahipliği bu fonksiyona geçti
    println!("take_ownership fonksiyonunda veri: {}", data);
    // Fonksiyon sona erdiğinde 'data' bellekten otomatik olarak temizlenir
}

fn copy_value(num: i32) {
    // 'num' değeri bu fonksiyona kopyalandı
    println!("copy_value fonksiyonunda numara: {}", num);
    // Fonksiyon sona erdiğinde 'num' hala geçerli ve kullanılabilir
}
