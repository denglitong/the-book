pub fn main() {
    list::exercise();
    string::exercise();
    hashmap::exercise();
}

mod list {
    use std::collections::HashMap;

    pub fn exercise() {
        let mut a = vec![1, 2, 9, 3, 4, 6, 5, 8, 7, 5, 6];
        println!("vec {:?}\nmean: {}", a, list_mean(&a));
        println!("median: {}", list_median(&mut a));
        println!("vec {:?}", a);
        println!("vec mode: {}", list_mode(&a));
    }

    fn list_mean(v: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in v {
            sum += *i;
        }
        sum / v.len() as i32
    }

    fn list_median(v: &mut Vec<i32>) -> i32 {
        v.sort();
        if v.len() % 2 == 0 {
            return (*v.get((v.len() - 1) / 2).unwrap() + *v.get((v.len() + 1) / 2).unwrap()) / 2;
        }
        return *v.get((v.len() + 1) / 2).unwrap();
    }

    fn list_mode(v: &Vec<i32>) -> i32 {
        let mut hashmap = HashMap::new();
        for i in v {
            *hashmap.entry(*i).or_insert(0) += 1;
            //*hashmap.get_mut(i).unwrap() += 1;
        }

        // println!("{:?}", hashmap);
        let mut mode = 0;
        let mut mode_count = 0;
        for x in hashmap {
            if x.1 > mode_count {
                mode_count = x.1;
                mode = x.0;
            }
        }

        mode
    }
}

mod string {
    use unicode_segmentation::UnicodeSegmentation;

    pub fn exercise() {
        let a = String::from("first");
        println!("{} {}", a, string_convert(&a));
        let a = String::from("apply");
        println!("{} {}", a, string_convert(&a));
    }

    fn string_convert(str: &String) -> String {
        let graphemes = UnicodeSegmentation::graphemes(str.as_str(), true).collect::<Vec<&str>>();
        match graphemes.get(0) {
            Some(&"a") | Some(&"e") | Some(&"i") | Some(&"o") | Some(&"u") => {
                let mut res = String::from(str);
                res.push_str("-fay");

                res
            }
            _ => {
                let mut res = String::from("");
                for &x in graphemes.iter().skip(1) {
                    res.push_str(x);
                }
                res.push_str("-");
                res.push_str(*graphemes.get(0).unwrap());
                res.push_str("ay");

                res
            }
        }
    }
}

mod hashmap {
    use std::collections::HashMap;

    pub fn exercise() {
        let mut department_employees: HashMap<String, Vec<String>> = HashMap::new();
        add_employee_by_text_interface(
            &mut department_employees,
            "Add Sally to Engineering".to_string(),
        );
        add_employee_by_text_interface(&mut department_employees, "Add Leon to Sales".to_string());
        add_employee_by_text_interface(&mut department_employees, "Add Amir to Sales".to_string());
        println!("{:?}", department_employees);
        println!(
            "{:?}",
            get_department_employees(&department_employees, "Sales".to_string()).unwrap()
        );
        println!("{:?}", get_all_people(&department_employees));
    }

    fn add_employee_by_text_interface(
        department_employees: &mut HashMap<String, Vec<String>>,
        text: String,
    ) {
        let instruction = text.split_whitespace().collect::<Vec<&str>>();
        if instruction.len() != 4 {
            std::panic!("invalid instruction, you should input: Add {employee} to {department}");
        }

        let employee = *instruction.get(1).unwrap();
        let department = *instruction.get(3).unwrap();
        department_employees
            .entry(String::from(department))
            .or_insert(Vec::new());
        department_employees
            .get_mut(department)
            .unwrap()
            .push(String::from(employee));
    }

    fn get_department_employees(
        department_employees: &HashMap<String, Vec<String>>,
        department: String,
    ) -> Option<Vec<String>> {
        match department_employees.get(department.as_str()) {
            Some(employees) => {
                let mut e = employees.clone();
                e.sort();
                Some(e)
            }
            None => None,
        }
    }

    fn get_all_people(department_employees: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
        let mut employees: Vec<String> = Vec::new();
        for x in department_employees {
            for e in x.1 {
                employees.push(e.clone());
            }
        }
        if employees.len() > 0 {
            employees.sort();
            return Some(employees);
        }
        None
    }
}
