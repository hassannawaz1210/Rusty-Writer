use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new () -> Self {
        LinkedList { head: None, tail: None }
    }

    fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node { value, next: None }));

        match &self.head {
            //if list is empty
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            },
            //list not empty
            Some(_) => {
                if let Some(last) = &self.tail {
                    last.borrow_mut().next = Some(new_node.clone());
                    self.tail = Some(new_node.clone());
                }
            },
        }
    }

    fn delete(&mut self){
    //if head and tail are the same, then set both to None
        if let Some(head) = &self.head {
            if let Some(tail) = &self.tail {
                if Rc::ptr_eq(head, tail) {
                    self.head = None;
                    self.tail = None;
                } else { // if head and tail are not the same, then find the last node and delete it
                    let mut current = head.clone();
                    //fetch the second last node
                    loop {
                        let next = current.borrow().next.clone();
                        match next {
                            Some(next_node) => {
                                if Rc::ptr_eq(&next_node, tail) {
                                    current.borrow_mut().next = None;
                                    self.tail = Some(current.clone());
                                    break;
                                } else {
                                    current = next_node;
                                }
                            },
                            None => break,
                        }
                    }
                    
                }
            }
        }
    }

    fn insertAt(&mut self, value: T, index: i32) {
    let mut current = self.head.clone();
    let mut i = 0;
    loop {
        match current {
            Some(current_node) => {
                //since we are stopping at index-1 node, we have to add a special case for index 0
                //otherwise we would be able to insert at index 0
                if index == 0 {
                    let new_node = Rc::new(RefCell::new(Node { value, next: None }));
                    new_node.borrow_mut().next = self.head.clone();
                    self.head = Some(new_node.clone());
                    break;
                } else

                if i == index-1 {
                    let new_node = Rc::new(RefCell::new(Node { value, next: None }));
                    let next = current_node.borrow().next.clone();
                    current_node.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().next = next;
                    break;
                } else {
                    current = current_node.borrow().next.clone();
                    i += 1;
                }
            },
            None => {
                //if list is empty, insert at index 0
                if index == 0 {
               self.insert(value);
                }
               break;
            }
        }        

    }
    }

    fn deleteAt(&mut self, index: i32) {
    let mut current = self.head.clone();
    let mut i = 0;
    loop {
        match current {
            Some(current_node) => {

                if index == 0 {
                    //two cases: head and tail are the same, or they arent
                    //case 1: head and tail are the same
                    if let Some(tail) = &self.tail {
                        if Rc::ptr_eq(&current_node, tail) {//current_node is pointing to head here
                            self.head = None;
                            self.tail = None;
                            break;
                        } else { //case 2: head and tail are not the same
                            let next = current_node.borrow().next.clone();
                            self.head = next;
                            break;
                        }
                    }
                } 
                
                if i == index-1 {
                    let next = current_node.borrow().next.clone();
                    if let Some(next_node) = next {
                        let next_next = next_node.borrow().next.clone();
                        current_node.borrow_mut().next = next_next;
                    }
                    break;
                } else {
                    current = current_node.borrow().next.clone();
                    i += 1;
                }
            },
            None => {
               break;
            }
        }        

    }
    
    }
}


fn main()
{

    let mut list = LinkedList::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.deleteAt(1);

    println!("After insertAt");
    println!("Head: {:#?}", list.head);
    println!("Tail: {:#?}", list.tail);

}