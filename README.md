# SnoopySnap DApp 🐶✨

**SnoopySnap DApp** - Cute Blockchain-Based Snoopy Photobooth Application on Stellar

---

# Project Description

SnoopySnap DApp adalah aplikasi photobooth bertema Snoopy yang dibangun di atas blockchain Stellar menggunakan Soroban SDK. Aplikasi ini memungkinkan pengguna mengambil, menyimpan, dan membagikan foto lucu bertema Snoopy secara aman dan terdesentralisasi langsung di blockchain.

Setiap foto memiliki identitas unik, caption, filter, frame Snoopy, dan timestamp yang tersimpan permanen di blockchain Stellar. Sistem ini memastikan seluruh kenangan digital tetap aman, transparan, dan tidak dapat dimanipulasi.

SnoopySnap menghadirkan pengalaman photobooth modern dengan nuansa lucu, hangat, dan nostalgic ala Snoopy & Peanuts universe 🐶💛

Aplikasi ini cocok digunakan untuk:

* Wedding photobooth
* Campus event booth
* Festival & konser
* NFT memory gallery
* Birthday party
* Community memories
* Web3 event photobooth

---

# Project Vision

Visi kami adalah menciptakan photobooth decentralized yang:

* **Lucu & menyenangkan**
* **Terdesentralisasi**
* **Aman**
* **Dimiliki penuh oleh pengguna**
* **Mudah digunakan di berbagai event**
* **Mengabadikan momen bahagia di blockchain**

Kami percaya bahwa kenangan digital adalah aset berharga yang harus dimiliki sepenuhnya oleh pengguna tanpa ketergantungan pada platform terpusat.

---

# Key Features

## 1. Upload Snoopy Photos

* Upload foto menggunakan URL/IPFS hash
* Penyimpanan permanen di Stellar blockchain
* ID unik otomatis untuk setiap foto
* Tema aesthetic Snoopy & Peanuts

---

## 2. Cute Captions & Timestamp

* Tambahkan caption lucu ala Snoopy
* Timestamp otomatis saat upload
* Dokumentasi kenangan secara real-time

Contoh caption:
* `"Happiness is taking pictures together 🐶"`
* `"Good vibes with Snoopy ✨"`

---

## 3. Snoopy Frames & Filters

Pilihan frame:
* Snoopy House
* Woodstock Theme
* Comic Strip Theme
* Charlie Brown Style

Pilihan filter:
* Vintage comic
* Soft pastel
* Cartoon mode
* Black & white peanuts

---

## 4. Decentralized Photo Gallery

* Menampilkan seluruh gallery photobooth
* Mudah diintegrasikan dengan React / Next.js
* Sinkronisasi langsung dengan blockchain

---

## 5. Delete Photos

* Menghapus foto berdasarkan ID
* Update storage otomatis
* Gallery lebih rapi dan terorganisir

---

## 6. Blockchain Transparency

* Semua aktivitas dapat diverifikasi
* Riwayat upload transparan
* Tidak dapat dimanipulasi pihak lain

---

# Smart Contract Features

## `upload_photo()`

Mengupload foto baru ke blockchain.

Parameter:

* `username`
* `photo_url`
* `caption`
* `frame`
* `filter`

---

## `get_photos()`

Mengambil seluruh daftar foto dari blockchain storage.

---

## `delete_photo()`

Menghapus foto berdasarkan ID.

---

# Data Structure

```rust
pub struct Photo {
    id: u64,
    username: String,
    photo_url: String,
    caption: String,
    frame: String,
    filter: String,
    timestamp: u64,
}