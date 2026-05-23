
#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Photo {
    id: u64,
    username: String,
    photo_url: String,
    caption: String,
    frame: String,
    filter: String,
    timestamp: u64,
}

const PHOTO_DATA: Symbol = symbol_short!("PHOTO");

#[contract]
pub struct SnoopySnapContract;

#[contractimpl]
impl SnoopySnapContract {

    pub fn get_photos(env: Env) -> Vec<Photo> {

        env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env))
    }

    pub fn upload_photo(
        env: Env,
        username: String,
        photo_url: String,
        caption: String,
        frame: String,
        filter: String,
    ) -> String {

        let mut photos: Vec<Photo> = env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env));

        let photo = Photo {
            id: env.prng().gen::<u64>(),
            username,
            photo_url,
            caption,
            frame,
            filter,
            timestamp: env.ledger().timestamp(),
        };

        photos.push_back(photo);

        env.storage().instance().set(&PHOTO_DATA, &photos);

        String::from_str(&env, "Snoopy photo uploaded!")
    }

    pub fn delete_photo(env: Env, id: u64) -> String {

        let mut photos: Vec<Photo> = env.storage()
            .instance()
            .get(&PHOTO_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..photos.len() {

            if photos.get(i).unwrap().id == id {

                photos.remove(i);

                env.storage().instance().set(&PHOTO_DATA, &photos);

                return String::from_str(
                    &env,
                    "Photo deleted!"
                );
            }
        }

        String::from_str(&env, "Photo not found")
    }
}

mod test;