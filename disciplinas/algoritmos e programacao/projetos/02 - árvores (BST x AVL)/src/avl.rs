use crate::employee::Employee;

pub struct Node {
    pub employee: Employee,
    pub height: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

pub struct AVL {
    pub root: Option<Box<Node>>,
}

impl AVL {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn height(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn update_height(node: &mut Box<Node>) {
        node.height = 1 + std::cmp::max(
            Self::height(&node.left),
            Self::height(&node.right),
        );
    }

    fn right_rotate(mut y: Box<Node>) -> Box<Node> {
        let mut x = y.left.take().unwrap();
        let t2 = x.right.take();

        y.left = t2;
        Self::update_height(&mut y);

        x.right = Some(y);
        Self::update_height(&mut x);

        x
    }

    fn left_rotate(mut x: Box<Node>) -> Box<Node> {
        let mut y = x.right.take().unwrap();
        let t2 = y.left.take();

        x.right = t2;
        Self::update_height(&mut x);

        y.left = Some(x);
        Self::update_height(&mut y);

        y
    }

    pub fn insert(&mut self, employee: Employee) {
        self.root = Self::insert_node(self.root.take(), employee);
    }

    fn insert_node(node: Option<Box<Node>>, employee: Employee) -> Option<Box<Node>> {
        let mut current = match node {
            Some(n) => n,
            None => {
                return Some(Box::new(Node {
                    employee,
                    height: 1,
                    left: None,
                    right: None,
                }))
            }
        };

        if employee.id < current.employee.id {
            current.left = Self::insert_node(current.left, employee.clone());
        } else {
            current.right = Self::insert_node(current.right, employee.clone());
        }

        // Atualiza altura
        Self::update_height(&mut current);

        let balance = Self::height(&current.left) - Self::height(&current.right);

        // LL
        if balance > 1 && employee.id < current.left.as_ref().unwrap().employee.id {
            return Some(Self::right_rotate(current));
        }

        // RR
        if balance < -1 && employee.id > current.right.as_ref().unwrap().employee.id {
            return Some(Self::left_rotate(current));
        }

        // LR
        if balance > 1 && employee.id > current.left.as_ref().unwrap().employee.id {
            current.left = Some(Self::left_rotate(current.left.take().unwrap()));
            return Some(Self::right_rotate(current));
        }

        // RL
        if balance < -1 && employee.id < current.right.as_ref().unwrap().employee.id {
            current.right = Some(Self::right_rotate(current.right.take().unwrap()));
            return Some(Self::left_rotate(current));
        }

        Some(current)
    }

    pub fn get_height(&self) -> i32 {
        Self::height(&self.root)
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