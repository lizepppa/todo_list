use serde::{Serialize, Deserialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String, description: String) -> Self {
        Task {
            id,
            title,
            description,
            completed: false,
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut title = String::new();
    let mut description = String::new();

    println!("Введіть назву завдання:");
    io::stdin()
        .read_line(&mut title)
        .expect("Не вдалося зчитати введення");
    let title = title.trim().to_string();

    println!("Введіть опис завдання:");
    io::stdin()
        .read_line(&mut description)
        .expect("Не вдалося зчитати введення");
    let description = description.trim().to_string();

    let id = if tasks.is_empty() {
        1
    } else {
        tasks.last().unwrap().id + 1
    };

    let new_task = Task::new(id, title, description);
    tasks.push(new_task);
    println!("Завдання успішно додано!");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("Завдань не знайдено!");
    } else {
        for task in tasks {
            println!(
                "ID: {}, Назва: {}, Опис: {}, Виконано: {}",
                task.id, task.title, task.description, task.completed
            );
        }
    }
}

fn edit_task(tasks: &mut Vec<Task>) {
    println!("Введіть ID завдання для редагування:");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Не вдалося зчитати введення");

    if let Ok(id) = id_input.trim().parse::<u32>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            println!("Редагування завдання: {:?}", task);
            println!("Введіть нову назву (залиште порожнім, щоб зберегти поточну):");
            let mut title = String::new();
            io::stdin().read_line(&mut title).expect("Не вдалося зчитати введення");
            let title = title.trim();
            if !title.is_empty() {
                task.title = title.to_string();
            }

            println!("Введіть новий опис (залиште порожнім, щоб зберегти поточний):");
            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("Не вдалося зчитати введення");
            let description = description.trim();
            if !description.is_empty() {
                task.description = description.to_string();
            }

            println!("Позначити як виконане? (так/ні):");
            let mut completed_input = String::new();
            io::stdin()
                .read_line(&mut completed_input)
                .expect("Не вдалося зчитати введення");
            if completed_input.trim().eq_ignore_ascii_case("так") {
                task.completed = true;
            }

            println!("Завдання успішно оновлено!");
        } else {
            println!("Завдання з ID {} не знайдено.", id);
        }
    } else {
        println!("Невірний ID.");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    println!("Введіть ID завдання для видалення:");
    let mut id_input = String::new();
    io::stdin()
        .read_line(&mut id_input)
        .expect("Не вдалося зчитати введення");

    if let Ok(id) = id_input.trim().parse::<u32>() {
        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
            tasks.remove(pos);
            println!("Завдання успішно видалено!");
        } else {
            println!("Завдання з ID {} не знайдено.", id);
        }
    } else {
        println!("Невірний ID.");
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string(tasks).expect("Не вдалося серіалізувати завдання");
    fs::write("tasks.json", json).expect("Не вдалося записати у файл");
    println!("Завдання успішно збережено!");
}

fn load_tasks() -> Vec<Task> {
    if let Ok(json) = fs::read_to_string("tasks.json") {
        serde_json::from_str(&json).unwrap_or_else(|_| {
            println!("Не вдалося зчитати файл завдань. Починаємо зі списку, що порожній.");
            Vec::new()
        })
    } else {
        println!("Файл завдань не знайдено. Починаємо зі списку, що порожній.");
        Vec::new()
    }
}

fn main() {
    let mut tasks = load_tasks();

    loop {
        println!("\nМеню завдань:");
        println!("1. Додати завдання");
        println!("2. Переглянути список завдань");
        println!("3. Редагувати завдання");
        println!("4. Видалити завдання");
        println!("5. Зберегти завдання");
        println!("6. Вийти");
        println!("Оберіть функцію:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Не вдалося зчитати введення");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => edit_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => save_tasks(&tasks),
            "6" => {
                save_tasks(&tasks);
                println!("Вихід із програми. До побачення!");
                break;
            }
            _ => println!("Неправильний вибір. Спробуйте ще раз."),
        }
    }
}
