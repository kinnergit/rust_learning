type NodePtr<T> = Option<Box<Node<T>>>;
struct Node<T>
{
    data: T,
    next: NodePtr<T>,
}

fn main() {
    let mut n1 = Box::new(Node{data: 1, next: None});
    let mut n2 = Box::new(Node{data: 2, next: None});

    n1.next = Some(n2);
    n2.next = Some(n1);
}


