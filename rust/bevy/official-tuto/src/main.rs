use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    // スタートアップは全てスケジュールの最初に実行される
    App::new()
        .add_systems(Startup, add_people)
        // chainは順番を保証する。hello_worldはご自由に
        .add_systems(Update, (hello_world, (update_people, greet_people).chain()))
        .run();
}

fn add_people(mut commands: Commands) {
    // Parsonに対してNameを持たせてワールドへスポーンさせる
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn hello_world() {
    println!("hello world!")
}

// queryはdata とフィルターを要求する。これはParsonが持っているNameデータをワールドから取り出す
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

// 変更するQuery mutで受け取る
fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
