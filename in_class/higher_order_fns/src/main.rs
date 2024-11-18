// // Define our data structure
// struct Data {
//     value: i32,
// }

// // Higher-order function: defines what needs to be done
// fn process_data(data: &mut [Data], operation: fn(&mut Data)) {
//     for item in data.iter_mut() {
//         operation(item);
//     }
// }

// // Specific operations: actual functions which do the work
// fn double_value(data: &mut Data) {
//     data.value *= 2;
// }

// fn square_value(data: &mut Data) {
//     data.value = data.value * data.value;
// }

// fn triple_value(data: &mut Data) {
//     data.value *= 3;
// }

// // Helper function to print values without closures
// fn print_values(items: &[Data]) {
//     print!("Values: ");
//     for (i, item) in items.iter().enumerate() {
//         if i > 0 {
//             print!(", ");
//         }
//         print!("{}", item.value);
//     }
//     println!();
// }

// fn main() {
//     let mut items = vec![
//         Data { value: 1 },
//         Data { value: 2 },
//         Data { value: 3 },
//         Data { value: 4 },
//         Data { value: 5 },
//     ];
    
//     // The specific operation is decided here
//     print!("Original ");
//     print_values(&items);
    
//     process_data(&mut items, double_value);
//     print!("After doubling: ");
//     print_values(&items);
    
//     // We can easily switch to a different operation
//     process_data(&mut items, square_value);
//     print!("After squaring: ");
//     print_values(&items);

//     process_data(&mut items, triple_value);
//     print!("After tripling: ");
//     print_values(&items);
// }

///////////////////////////////////////////////////////

// fn add(x: i32, y: i32) -> i32 {
//     return x + y;
// } 

// fn mul(x: i32, y: i32) -> i32 {
//     return x * y;
// } 

// struct Calculator {
//     addition: fn(x: i32, y: i32) -> i32,
//     multiplication: fn(x: i32, y: i32) -> i32,
// }

// impl Calculator {
//     fn new(addition_behaviour: fn(x: i32, y: i32) -> i32,
//            multiplication_behaviour: fn(x: i32, y: i32) -> i32
//         ) -> Calculator {
//         return Calculator {
//             addition: addition_behaviour,
//             multiplication: multiplication_behaviour
//         }
//     }

//     fn add(&self, x: i32, y: i32) -> i32 {
//         return (self.addition)(x, y);
//     }

//     fn mul(&self, x: i32, y: i32) -> i32 {
//         return (self.multiplication)(x, y);
//     }
// }

// fn main() {
//     let c = Calculator::new(add, mul);
//     let r = c.add(5, 10);
//     println!("{}", r);
//     let r = c.mul(5, 10);
//     println!("{}", r);
// }

struct DataRecord {
    id: u32,
    value: String,
}

fn extract(_source: &str) -> Vec<DataRecord> {
    vec![
        DataRecord { id: 1, value: String::from("data1") },
        DataRecord { id: 2, value: String::from("data2") },
    ]
}

fn transform_uppercase(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: record.value.to_uppercase() }
}

fn transform_add_prefix(record: DataRecord) -> DataRecord {
    DataRecord { id: record.id, value: format!("PREFIX_{}", record.value) }
}

fn load(records: Vec<DataRecord>) {
    for record in records {
        println!("Loaded: id={}, value={}", record.id, record.value);
    }
}

struct EtlPipeline {
    extract: fn(&str) -> Vec<DataRecord>,
    transform1: fn(DataRecord) -> DataRecord,
    transform2: fn(DataRecord) -> DataRecord,
    load: fn(Vec<DataRecord>),
}

impl EtlPipeline {
    fn new(
        extract: fn(&str) -> Vec<DataRecord>,
        transform1: fn(DataRecord) -> DataRecord,
        transform2: fn(DataRecord) -> DataRecord,
        load: fn(Vec<DataRecord>)
    ) -> Self {
        EtlPipeline { extract, transform1, transform2, load }
    }

    fn run(&self, source: &str) {
        let data = (self.extract)(source);
        let transformed_data: Vec<_> = data.into_iter()
            .map(self.transform1)
            .map(self.transform2)
            .collect();
        (self.load)(transformed_data);
    }
}

fn main() {
    let etl_pipeline = EtlPipeline::new(extract, transform_uppercase, transform_add_prefix, load);
    etl_pipeline.run("dummy_source");
}