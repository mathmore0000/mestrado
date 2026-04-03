use crate::employee::Employee;

pub struct Node {
    pub employee: Employee,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct BST {
    pub root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, employee: Employee) {
        self.root = Self::insert_node(self.root.take(), employee);
    }

    fn insert_node(node: Option<Box<Node>>, employee: Employee) -> Option<Box<Node>> {
        match node {
            Some(mut current) => {
                if employee.id < current.employee.id {
                    current.left = Self::insert_node(current.left, employee);
                } else {
                    current.right = Self::insert_node(current.right, employee);
                }
                Some(current)
            }
            None => Some(Box::new(Node {
                employee,
                left: None,
                right: None,
            })),
        }
    }

    pub fn search(&self, id: i32) -> Option<&Employee> {
        Self::search_node(&self.root, id)
    }

    fn search_node(node: &Option<Box<Node>>, id: i32) -> Option<&Employee> {
        match node {
            Some(current) => {
                if id == current.employee.id {
                    Some(&current.employee)
                } else if id < current.employee.id {
                    Self::search_node(&current.left, id)
                } else {
                    Self::search_node(&current.right, id)
                }
            }
            None => None,
        }
    }

    pub fn height(&self) -> i32 {
        Self::height_node(&self.root)
    }

    fn height_node(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(current) => {
                1 + std::cmp::max(
                    Self::height_node(&current.left),
                    Self::height_node(&current.right),
                )
            }
            None => 0,
        }
    }

    pub fn find_by_salary(&self, salary: i32) -> Vec<&Employee> {
        let mut result = Vec::new();
        Self::find_salary_node(&self.root, salary, &mut result);
        result
    }

    fn find_salary_node<'a>(
        node: &'a Option<Box<Node>>,
        salary: i32,
        result: &mut Vec<&'a Employee>,
    ) {
        if let Some(current) = node {
            if current.employee.salary == salary {
                result.push(&current.employee);
            }

            Self::find_salary_node(&current.left, salary, result);
            Self::find_salary_node(&current.right, salary, result);
        }
    }

}