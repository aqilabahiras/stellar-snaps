
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec,
};

// Struktur data photo
#[contracttype]
#[derive(Clone, Debug)]
pub struct Photo {
    id: u64,
    photo_url: String,
    caption: String,
    timestamp: u64,
}

// Storage key
const PHOTO_DATA: Symbol = symbol_short!("PHOTO");

// Contract utama
#[contract]
pub struct PhotoboothContract;

#[contractimpl]
impl PhotoboothContract {

    // Ambil semua foto
    pub fn get_photos(env: Env) -> Vec<Photo> {
        env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Upload foto baru
    pub fn upload_photo(
        env: Env,
        photo_url: String,
        caption: String,
    ) -> String {

        // Ambil data lama
        let mut photos: Vec<Photo> = env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat photo baru
        let photo = Photo {
            id: env.prng().gen::<u64>(),
            photo_url,
            caption,
            timestamp: env.ledger().timestamp(),
        };

        // Simpan ke vector
        photos.push_back(photo);

        // Save ke storage
        env.storage().instance().set(&PHOTO_DATA, &photos);

        String::from_str(&env, "Foto berhasil diupload")
    }

    // Hapus foto berdasarkan id
    pub fn delete_photo(env: Env, id: u64) -> String {

        let mut photos: Vec<Photo> = env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..photos.len() {
            if photos.get(i).unwrap().id == id {

                photos.remove(i);

                env.storage().instance().set(&PHOTO_DATA, &photos);

                return String::from_str(&env, "Foto berhasil dihapus");
            }
        }

        String::from_str(&env, "Foto tidak ditemukan")
    }
}

mod test;