use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use user::schema::users;

#[table_name = "users"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub password: String
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");

        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read(id: i32, connection: &MysqlConnection) -> Vec<User> {
        if id != 0 {
            users::table.find(id).order(users::id).load::<User>(connection).unwrap()
        } else {
            users::table.order(users::id).load::<User>(connection).unwrap()
        }
    }

    pub fn byUsernameAndPassword(username_: String, password_: String, connection: &MysqlConnection) -> User {
        let res = users::table
            .filter(users::name.eq(username_))
            .filter(users::password.eq(password_))
            .order(users::id)
            .first(connection);
        match res {
            Ok(user) => user,
            Err(err) => {
                User {
                    id: None,
                    name: String::from(""),
                    password: String::from("")
                }
            }
        }
    }

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}
