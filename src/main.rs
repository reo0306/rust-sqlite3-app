use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: u16,
    department: String,
    name: String,
    salary: u32,
    avg_salary: Option<f32>,
}

fn open_my_db() -> Result<Connection, rusqlite::Error> {
    let path = "./db/rust_db.db3";
    let con = Connection::open(&path)?;
    println!("{}", con.is_autocommit());

    Ok(con)
}

fn insert_person(con: &Connection, p: &Person) -> Result<usize, rusqlite::Error> {
    Ok(
        con.execute(
            "insert into person (department, name, salary) values (?1, ?2, ?3)",
            params![p.department, p.name, p.salary]
        )?
    )
}

fn select_all(con: &Connection) {
    let mut stmt = con.prepare(
        "select id, department, name, salary, avg(salary) over defw from person window defw as (partition by department) order by id")
        .unwrap();

    let persons = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            department: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            salary: row.get(3).unwrap(),
            avg_salary: Some(row.get_unwrap(4)),
        })
    }).unwrap();

    persons.into_iter().for_each(|person| {
        let p = person.unwrap();
        println!("{}", format!("id:{}, department:{}, name:{}, salary:{}, avg_salary:{}", p.id, p.department, p.name, p.salary, p.avg_salary.unwrap()));
    });
}

fn main() {
    let con = open_my_db().unwrap();

    let person = Person {
        id: 11,
        department: "development".to_string(),
        name: "Shota".to_string(),
        salary: 300,
        avg_salary: None,
    };

    let _ = insert_person(&con, &person);
    select_all(&con);
}
