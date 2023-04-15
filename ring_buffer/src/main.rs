struct RingBuffer {
    data: Vec<i8>,
    head: usize,
    tail: usize,
    len: usize,
}

impl RingBuffer {
    fn new() -> Self {
        Self {
            data: vec![0; 5],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    fn get_needed_array_size(&self) -> usize {
        self.data.len() + 5
    }

    fn expand_array(&mut self) {
        println!("Setting array size to {}...", self.get_needed_array_size());
        let mut i = self.head;
        let mut new_index = 0;
        let mut new_data: Vec<i8>= vec![Default::default(); self.get_needed_array_size()];

        while i != self.tail {
            new_data[new_index] = self.data[i];
            i = (i + 1) % self.data.len();
            new_index += 1;
        }
        self.data = new_data;
        self.head = 0;
        self.tail = new_index;
    }

    fn push(&mut self, value: i8) {
        self.len += 1;
        println!("pushing {} at {}", value, self.tail);
        if self.len == self.data.len() {
            self.expand_array();
        }

        self.data[self.tail] = value;
        self.tail = (self.tail + 1) % self.data.len();
    }

    fn pop(&mut self) {
        if self.len == 0 {
            println!("cant pop RingBuffer is empty");
            return;
        }
        println!("popd {}", self.data[self.head]);

        self.head = (self.head + 1) % self.data.len();
        self.len -= 1;
    }

    fn print(&self) {
        let mut i = self.head;
        if self.len == self.data.len() {
            return println!("RingBuffer is empty");
        }
        print!("RingBuffer: ");
        while i != self.tail {
            print!("{}, ", self.data[i]);
            i = (i + 1) % self.data.len();
        }
        println!();
    }
}

fn main() {
    let mut ring_buffer = RingBuffer::new();
    // ring_buffer.print();
    ring_buffer.push(1);
    ring_buffer.push(2);
    ring_buffer.push(3);
    // ring_buffer.print();
    ring_buffer.push(4);
    ring_buffer.push(5);
    ring_buffer.push(6);
    ring_buffer.print();
    ring_buffer.pop();
    ring_buffer.push(0);
    ring_buffer.pop();
    ring_buffer.print();
    ring_buffer.push(7);
    ring_buffer.push(8);
    ring_buffer.push(9);
    ring_buffer.push(10);
    ring_buffer.push(11);
    ring_buffer.push(12);
    ring_buffer.print();
    // ring_buffer.print();
    // ring_buffer.pop();
    // ring_buffer.pop();
    // ring_buffer.pop();
    // ring_buffer.pop();
    // ring_buffer.pop();
    // ring_buffer.print();
    // ring_buffer;
}
