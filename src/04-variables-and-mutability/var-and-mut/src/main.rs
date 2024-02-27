#[allow(unused_doc_comments)]

fn main() {
    /// # Variables & Mutability
    ///
    /// - If you try to reassign a variable that is not mutable, you will get a compile-time error.
    /// - The error message will tell you that the variable is not mutable.
    /// - To fix the error, you can add the mut keyword to the variable name to make it mutable.
    ///
    /// # Değişkenler ve Değiştirilebilirlik
    ///
    /// - Değiştirilemeyen bir değişkeni yeniden atamaya çalışırsanız, derleme esnasında hata alırsınız.
    /// - Hata mesajı, değişkenin değiştirilemez olduğunu bildirir.
    /// - Hatayı düzeltmek için değişken adına mut anahtar kelimesini ekleyerek değiştirilebilir hale getirebilirsiniz.
    ///
    /// ```rust
    /// let x = 5;      // Not assignable
    /// let mut x = 5;  // Assignable
    /// ```
    println!(">> Variables & Mutability <<");

    let mut x = 5;
    println!(" The value of x is: {x}");

    x = 6;
    println!(" The value of x is: {x}");
    println!();

    /// # Constants
    ///
    /// - Constants are always immutable.
    /// - You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated.
    /// - Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of the program need to know about.
    /// - Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.
    ///
    /// # Sabitler
    ///
    /// - Constant'lar değiştirilemez.
    /// - let anahtar kelimesi yerine const anahtar kelimesini kullanarak bildirirsiniz ve değerin türü belirtilmelidir.
    /// - Programın birçok bölümünün bilmesi gereken değerler için kullanışlı olmaları nedeniyle, global kapsam dahil olmak üzere herhangi bir kapsamda bildirilebilir.
    /// - Yalnızca sabit bir ifadeye ayarlanabilir, bir işlev çağrısının sonucu veya yalnızca çalışma zamanında hesaplanabilecek başka bir değer değil.
    println!(">> Constants <<");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(" Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
    println!();

    /// # Shadowing
    ///
    /// - Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we forget to add the let keyword.
    /// - By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
    /// - The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
    ///
    /// # Gölgeleme
    ///
    /// - Bir değişkeni mut olarak işaretlemekten farklıdır, çünkü let anahtar kelimesini eklemeyi unutursak derleme sırasında hata alırız.
    /// - let kullanarak, bir değer üzerinde birkaç dönüşüm gerçekleştirebiliriz, ancak bu dönüşümler tamamlandıktan sonra değişken değiştirilemez olabilir.
    /// - mut ve shadowing arasındaki diğer fark, let anahtar kelimesini tekrar kullandığımızda etkili bir şekilde yeni bir değişken oluşturduğumuzdan değerin türünü değiştirebilmemiz ancak aynı adı yeniden kullanabilmemizdir.
    println!(">> Shadowing <<");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!(" The value of y in the inner scope is: {y}");
    }

    println!(" The value of y is: {y}");
}