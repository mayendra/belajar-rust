
fn main() {
    println!("Hello, world!");
}

// UNIT TEST
#[test] // atribute atau annotation
fn hello_test() {
    println!("hello world!");
}

// VARIABLE
/*
 * variable adalah tempat untuk menyimpan data
 * cara membuat vaiable di rust bisa menggunakan kata kunci let
 * setelah variable diisi data, maka tidak bisa bisa diubah lagi datanya (immutable)
*/
#[test]
fn test_variable() {
    let name = "mayendra dwika";
    println!("{}", name);
}

// MUTABLE
/*
   * seperti yang sudah dijelaskan sebelumnya, variable yang sudah diisi data tidak bisa diubah lagi, atau disebut Immutable
   * namun rust juga memperbolehkan jika ingin mengubah data variable , atau disebut Mutable
   * caranya kita bisa gunakan kata kunci let mut ketika membuat variable
*/
#[test]
fn test_mutable() {
    let mut name = "mayendra dwika";
    println!("{}", name);

    name = "mayendra prayudha";
    print!("{}", name);
}

// STATIC TYPING
/*
 * rust adalah bahasa static typing, artinya setiap membuat varible dengan tipe data tertentu, maka tidak akan bisa berubah menjadi tipe data lainnya
 * Sebelumnya kita mmebuat varible dengna tipe data text/string, kita tidak bisa mengubah tipe data tersebut menjadi int/number
*/
#[test]
fn test_static_typing() {
    // let mut name = "mayendra dwika";
    // println!("{}", name);

    // name = 10;
    // println!("{}",name); // ERROR
}

// SHADOWING
/*
 * di rust kita bisa membuat variable dengan nama yang sama
 * namun, saat kita membuat varible dengan nama yang sama, maka varible sebelumnya akan tertutup atau disbebut shadowing
 * praktek ini mungkin kurang baik  jika dilakukan jika terlalu sering, karena akan membingungkan yang membaca kode kita
 * namun ini diperbolehkan di rust
*/
#[test]
fn test_shadowing() {
    let name = "mayendra dwika";
    println!("{}", name);

    let name = "mayendra prayudha"; // SHADOWING
    println!("{}", name);
}

// DATA TYPE
/*
  * setiap nilai di Rust memiliki tipe data.
  * secara garis beras Rust membagi tipe data menjadi dua subsets; scalar compound
  * Scalar type merepresentasikan  single value(nilai tunggal) , yaitu integer, float, boolean, dan char
  * Compound type merepresentasikan beberapa value (bisa lebih dari satu) dalam satu tipe, yaitu tuple dan array
*/

// SCALAR TYPE
/*
 * integer type, tipe data number dalam bilangan bulat
 * float type, tipe data number dalam desmial
 * Boolean type, tipe data True or False
 * Char type, tipe data karakter
*/

// COMPOUND TYPE
/*
 * Tuple type, kumpulan beberapa data yang bisa berbeda tipe data
 * Array type, kumpulan beberapa data yang harus bertipe data sama
*/

// EXPLICIT TYPE
/*
 * saat membuat variable, secara default kita tidak perlu menyebutkan tipe data secara explicit, karena Rust bisa otomatismendeteksi tipe data apa yang kita gunakan
 * Namun, jika kita mau, kita juga bisa menyebutkan tipe data sebuah variable secara explicit dengan menggunakan tanda : (titik dua)
*/
#[test]
fn test_explicit() {
    let age : i32 = 20;
    println!("{}", age);

}

// DEFAULT NUMBER
/*
 * saat membuat variable secara implicit (tidak menyebutkan tipe data), maka rust akan menggunakan default type
 * jika bilangan bulat , maka akan menggunakan i32
 * jika bilangan pecahaan desimal , maka akan menggunakan f64
*/
#[test]
fn test_number() {
    let integer : i32 = 20;
    println!("{}", integer);

    let float : f64 = 3.14;
    println!("{}", float);
}

// KONVERSI TIPE DATA NUMBER
/*
 * rust bisa melakukan konversi tipe data dari tipe data number yang ukurannya kecil ke ukurannya lebih besar begitu juga sebaliknya
 * namun perlu diperhatikan , jika kita lakukan konversi tipe data number dari ukuran besar ke yang kecil, maka bisa terjadi yang namanya INTEGER OVERFLOW, yaitu kondisi dimana nilai number tidak bisa ditampung oleh tipe data tujuan konversi
 * Misal kita punya number 1000000 dalam bentuk i32, lalu kita konversi ke bentuk i8, maka akan terjadi intger overflow karena i8 tidak bisa menampung nilai tersebut
 * untuk melakukan konversi, kita bisa gunakan kata kunci as
*/
#[test]
fn test_convert_number() {
    let a : i8 = 20;
    println!("{}", a);

    let b : i16 = a as i16; // konversi dari i8 menjadi i16
    println!("{}", b);

    let c : i32 = a as i32; // konversi dari i16 menjadi i32
    println!("{}", c);
}

#[test]
fn test_boolean_operator() {
    let absen = 90;
    let nilai_akhir = 100;

    let lulus_absen = absen >= 75;
    let lulus_nilai = nilai_akhir >= 80;

    let lulus_final = lulus_absen && lulus_nilai;
    println!("{}", lulus_final);
}

// CHAR
/*
 * char adalah tipe data karakter
 * dibuat mengggunakan tanda petik satu (')
*/
#[test]
fn test_char() {
    let char1 : char = 'a'; // hanya satu
    let char2 : char = '4';
    let char3 : char = '@';

    println!("{} {} {}", char1, char2, char3);
}

// TUPLE
/*
 * adalah tipe data kumpulan lebih dari satu tipe data
 * jumlah data tuple sudah final, artinya tidak bisa berkurang atau bertambah
 * jika kita membuat tuple degan 3 data, maka tidak akan bisa diubah lagi jumlah data dan juga tipe data nya
 * untuk membuat tipe data tuple , kita bisa gunakan () tanda kurung
*/
#[test]
fn test_tuple() {
    let data_tuple : (i32, f64, bool) = (100, 3.14, false);
    println!("{:?}", data_tuple); // mengakses semua
    println!("{}", data_tuple.0); // mengakses nilai pertama
}

// DESCRUCTURING TUPLE
/*
 * kadang ketika kita ingin mengakses seluruh data tuple dan menyimpan nya di variable , akan sangat menyulitkan jika mengakses nya satu persatu menggunkaan normor index nya
 * tuple mendukung Destructuring , yaitu membongkar isi tuple dan menyimpannya di variable
 * jika ada data di tuple yang tidak kita butuhkan, kita bisa gunakan tanda _ (garis bawah) ketika melakukan destructuring tuple
*/
#[test]
fn test_destructuring() {
    let tuple: (i32, f64, bool) = (100, 3.14, false);
    println!("{:?}", tuple);

    let (a,b,c) = tuple; // destructuring
    println!("{} {} {}", a, b, c);
}

// MUTABLE TUPLE
/*
 * saat kita membuat tuple, secara otomatis tuple yang sudah dibuat tidak bisa diubah lagi datanya
 * namun jika kita membuat tuple tersebut sebagai mutable, maka data pada tuple tersebut dapat diubah datanya
 * untuk mengubah data pada tuple cukup gunakan nomor index dan = (sama dengan)
 * mirip seperti mengubah data variable biasa nya
*/
#[test]
fn test_mutable_tuple() {
    let mut tuple : (i32, f64, bool) = (100, 3.14, false);
    println!("{:?}", tuple);

    tuple.0 = 88;
    println!("{}", tuple.0);
    println!("{:?}", tuple); // menghasilakn output (88, 3.14, false)
}

// ARRAY
/*
 * array adalah tipe data yang berisi kumpulan data, sama seperti tuple
 * yang membedakan dengan tuple adalah pada array kita hanya bisa menggunakan satu tipe data
 * untuk membuat array mneggunakan [] tanda kurung kotak
*/
#[test]
fn test_array() {
    let array : [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);
}

// MENGAKSES ARRAY
/*
 * untuk mengakses array, sama seperti tuple, kita perlu tentukan nomor index yang akan diakses dimulai dari 0
 * namun cara mengaksesnya berbeda, tidak menggunakan . (titik) melainkan menggunakan [index]
*/
#[test]
fn test_accces_array() {
    let array1 : [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array1);

    // mengakses array
    let a = array1[0];
    let b = array1[1];
    let c = array1[2];
    println!("{} {} {}", a, b, c);
}

// MUTABLE ARRAY
/*
 * sama seperi tuple, jika kita membuat variable array dalam bentuk immutable, sudah pasti data nya tidak bisa diubah
 * jika ingin mengubah data pada array nya maka harus membuat dalam bentuk mutable
 * kita bisa mengubah data pada array dengan menggunakan [index] = diisi dengan nilai yang baru
*/
#[test]
fn test_mutable_array() {
    let mut array1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array1);

    array1[0] = 10;
    println!("{:?}", array1); // output [10,3,4,5]
}

// TWO DIMENSIONAL ARRAY
/*
 * saat membuat array, kita bisa menggunakan tipe data apapun di dalam array, bahkan jika didalam nya adalah tipe data array lagi
*/
#[test]
fn test_two_dimensional_array() {
    let matrix : [[i32;2];2] = [
        [1,2],
        [2,3],
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0]); // 1
    println!("{}", matrix[0][1]); // 2
    println!("{}", matrix[1][0]); // 2
    println!("{}", matrix[1][1]); // 3
}

// CONSTANT
/*
 * constant adalah variable immutable menggunakan kata kunci const
 * yang membedakan const dan let adalah , const tidak memiliki mutable, selain itu nilai const harus dideklarasikan ketika kode program dibuat(bukan dijalankan), oleh karena itu nilai const tidak bisa hasil kalkulasi nilai lain yang belu jelas hasilnya
 * untuk membuat const, wajib disebutkan tipe datanya secara explicit
 * nama const di Rust harus huruf besar dan biasanya pemisah kata menggunakan _ (underscore)
*/
#[test]
fn test_const() {
    const PI : f64 = 3.14;
    println!("{}", PI);
}

// VARIABLE SCOPE
/*
 * variable scope mendefinisikan area dimana varible bisa digunakan
 * variable bisa digunakan di dalam scope tempat definisi variable, dan juga di inner scope
 * namun variable tidak bisa digunakan di outer scope
*/
#[test]
fn test_variable_scope() {
    let angka : i32 = 10;

    { // innner scope
        println!("{}", angka);
        let angka_lagi : i32 = 100;
        println!("{}", angka_lagi);
    }
    // println!("{}", angka_lagi); // error karena berada diluar scope
}

// &str dan String
/*
 * rust memiliki tipe data text yang fixed size , yaitu &str (string slice) dan yang bisa mengembang/bertambah ukurannya, yaitu String
 * &str karena ukurannya fixed size, jadi rust akan menyimpannyya di Stack sedangkan String karena bisa bertambah/mengembang maka disimpan di Heap
*/

// IMMUTABLE STR
/*
 * karena ukuran &str adalah fixed size, maka operasi &str adalah tipe data yang immutable, artinya isi data &str tidak bisa diubah
 * ketika kita buat variable mutable, dan mengubah data &str, sebenarnya yang dilakukan adalah menggganti isi variable, bukan mengubah isi dari &str
 * &str memiliki banyak sekali method yang bisa digunakan untuk memanipulasi &str nya, namun akan menghasilkan nilai &str baru
 * namun perlu diperhatikan, beberapa method dari &str akan mengembalikan bentuk data String, bukan &str
*/


// STRING
/*
 * String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya
 * ketika kita buat dalam bentuk immmutable variable, maka String tidak bisa berkembang, namun tetap disimpan di Heap
 * ketika kita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
 * String juga memiliki method/function untuk memanipulsai data, namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri, ada juga method yan digunakan untuk mengubah dalam bentuk data baru, tanpa memodifikasi data aslinya
*/

// OWNERSHIP
/*
 * rust menggunakan ownership untuk melakukan data management di memory
 * Ownership adalah salah satu fitur unik di rust yang mungkin jarang ada di bahasa lain
 * ownership wajib dimengerti, karena akan berdampak ke hampir semua fitur di rust
 * ownership adalah fitur yang digunkaan oleh rust untuk menjadikan rust menjadi bahasa pemrograman yang aman dalam mengelola data di memory, tanpa harus adanya fitur Garbage Colllection atau Manual Memory Management
 * karena ownership adalah konsep yang baru untuk kebanyakan programmer, maka kadang butuh waktu untuk memamahaminya
*/

// OWNERSHIP RULES
/*
 * setiap value di rust harus punya owner (variable pemilik value)
 * dalam satu waktu, hanya bole ada satu owner
 * ketika owner keluar scope, value akan dihapus
*/
#[test]
fn ownership_test() {
    // a tidak bisa diakses , karena belum dideklarasikan
    let a = 10;

    { // b tidak bisa diakses disin karena belum dideklrasikan
        let b  = 60;
        println!("{}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi


// DATA COPY
/*
 * sesuai aturan di Ownership Rules, setipa value harus dimiliki oleh satu owner pada satu waktu
 * ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
 * semua data yang bersifat fixed size (yang disimpan di stack), ketika itu ditambahkan ke variable berbeda (owner baru0, maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data asil copy dari variable lama (owner lama)
 * oleh karena itu , tiap data akan selali dimiliki oleh satu owner pada satu waktu
*/
#[test]
fn data_copy_test() {
    let a = 10;
    let b = a; // b akan memiliki nilai 10 juga

    println!("{} {}", a, b);
}

// OWNERSHIP MOVEMENT
/*
 * namun data copy tidak terjadi untuk tipe data yang dismpan di heap
 * seperti aturan di Ownership, dalam satu waktu value hanya dimiliki satu owner
 * maka ketika kita coba buat varibale baru (owner baru) dari varible lama (owner lama), maka yang terjadi bukanlah copy, melainkan transfer ownership dari owner lama ke owner baru
 * setelah proses transfer selesai, secara otomatis owner lama akan dianggap tidak valid lagi digunakan
*/
#[test]
fn test_movement_ownership() {
    let name1 = String::from("mayendra");

    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses lagi
    println!("{}", name2);
}

// CLONE
/*
 * sekarang kita tahu bahwa data di Stack akan di cpoy sedangkan data do hesp akan dipindahkan ownership nya
 * lantas bagaimana jika kita ingin melakukan copy untuk data di Heap?
 * maka kita harus melakukan CLone
 * Clone adalah membuat data tiruan yang sama dari data salinya
 * String memiliki method clone() untuk melakukan ini
 * saat kita ingin memanggil methpd clone() maka method tersebut akan meng copy data String menjadi data String data baru
 * semua tipe data yang disimpan di Heap di rust memiliki method clone()
*/
#[test]
fn test_clone_ownership() {
    let name1 = String::from("mayendra");
    // maka akan ada dua data
    let name2 = name1.clone();
    println!("{}", name1);
    println!("{}", name2);
}

#[test]
fn if_expr_test() {
    let value = 90;
    let result = if value >= 90  {
        "very good"
    } else if value >= 80  {
        "good"
    } else if value >= 70  {
        "nice"
    } else {
        "bad"
    };

    println!("{}", result);
}