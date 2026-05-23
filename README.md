# Stellar Photobooth DApp

**Stellar Photobooth DApp** - Blockchain-Based Decentralized Photo Sharing Application

## Project Description

Stellar Photobooth DApp adalah aplikasi photobooth terdesentralisasi yang dibangun di atas blockchain Stellar menggunakan Soroban SDK. Aplikasi ini memungkinkan pengguna untuk mengambil, menyimpan, dan membagikan foto secara aman langsung di blockchain tanpa bergantung pada server terpusat.

Setiap foto memiliki identitas unik, caption, timestamp, dan dapat disimpan secara permanen di blockchain. Sistem ini memastikan bahwa semua foto bersifat transparan, aman, dan tidak dapat dimanipulasi oleh pihak ketiga.

Photobooth ini cocok digunakan untuk:

* Event digital
* Wedding booth
* Konser & festival
* NFT photo gallery
* Kenangan komunitas berbasis blockchain

---

# Project Vision

Visi kami adalah membangun platform photobooth modern yang:

* **Terdesentralisasi**
* **Aman**
* **Dimiliki penuh oleh pengguna**
* **Tidak bergantung pada cloud tradisional**
* **Mudah digunakan di berbagai event**

Kami percaya bahwa foto digital adalah aset berharga yang seharusnya dimiliki sepenuhnya oleh pengguna, bukan platform terpusat.

---

# Key Features

## 1. Upload Foto ke Blockchain

* Upload foto menggunakan URL/IPFS hash
* Penyimpanan permanen di Stellar blockchain
* Setiap foto memiliki ID unik otomatis

## 2. Caption & Timestamp

* Tambahkan caption pada setiap foto
* Timestamp otomatis saat upload
* Dokumentasi kenangan secara real-time

## 3. Ambil Semua Foto

* Menampilkan seluruh gallery photobooth
* Mudah diintegrasikan dengan frontend React/Next.js
* Sinkronisasi langsung dengan blockchain

## 4. Hapus Foto

* Menghapus foto berdasarkan ID
* Update storage secara otomatis
* Manajemen gallery lebih rapi

## 5. Transparansi Blockchain

* Semua aktivitas dapat diverifikasi
* Riwayat upload transparan
* Tidak bisa dimanipulasi pihak lain

---

# Smart Contract Features

## `upload_photo()`

Digunakan untuk mengupload foto baru ke blockchain.

Parameter:

* `photo_url`
* `caption`

## `get_photos()`

Mengambil seluruh daftar foto dari storage blockchain.

## `delete_photo()`

Menghapus foto berdasarkan ID.

---

# Data Structure

```rust
pub struct Photo {
    id: u64,
    photo_url: String,
    caption: String,
    timestamp: u64,
}
```

---

# Future Scope

## Short-Term Features

### 1. Camera Integration

* Ambil foto langsung dari webcam
* Countdown timer photobooth
* Auto capture

### 2. Photo Filters

* Vintage filter
* Black & white
* Neon effect
* AI beautify

### 3. QR Download

* Generate QR code untuk download foto
* Share ke media sosial

### 4. IPFS Storage

* Integrasi IPFS/Filecoin
* Storage lebih murah dan scalable

---

## Medium-Term Development

### 5. NFT Photobooth

* Convert foto menjadi NFT
* Mint langsung di Stellar

### 6. Event Mode

* Multiple event gallery
* Event code system
* Guest access

### 7. Like & Comment System

* Interaksi antar pengguna
* Social photobooth experience

### 8. Wallet Integration

* Freighter Wallet
* Stellar Wallet Kit
* User authentication

---

## Long-Term Vision

### 9. AI Photobooth

* AI background replacement
* AI pose suggestion
* AI face enhancement

### 10. Metaverse Integration

* Display photo gallery di virtual world
* Avatar photobooth

### 11. DAO Governance

* Voting fitur komunitas
* Event ownership decentralized

### 12. Decentralized CDN

* Fully decentralized image delivery
* Global access performance

---

# Technical Requirements

* Rust
* Soroban SDK
* Stellar Blockchain
* IPFS (optional)
* React / Next.js frontend
* Freighter Wallet

---

# Getting Started

Deploy smart contract ke jaringan Soroban lalu gunakan fungsi berikut:

* `upload_photo()` → upload foto baru
* `get_photos()` → ambil semua foto
* `delete_photo()` → hapus foto berdasarkan ID

---

# Example Use Cases

* Wedding Photobooth
* Campus Event Gallery
* NFT Selfie Booth
* Community Memories
* Blockchain Photo Album
* Festival Digital Gallery

---

# Contract Details

* Network: Stellar Soroban
* Language: Rust
* Smart Contract Type: Decentralized Photo Storage

---

# Conclusion

**Stellar Photobooth DApp** menghadirkan pengalaman photobooth modern berbasis blockchain yang aman, transparan, dan sepenuhnya dimiliki pengguna.

Dengan kekuatan Stellar dan Soroban SDK, aplikasi ini membuka era baru digital memories yang truly decentralized.
